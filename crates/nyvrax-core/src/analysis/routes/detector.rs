use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use regex::Regex;

use crate::{DiffHunk, ScanContext};

use super::{HttpMethod, RouteState, RouteSurface};

static ROUTE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)\b(?:router|app)\s*\.\s*(get|post|put|patch|delete|options|head)\s*\(")
        .expect("valid route regex")
});

#[derive(Debug)]
struct ParsedRoute {
    method: HttpMethod,
    path: String,
    handlers: Vec<String>,
    line_offset: usize,
}

pub fn detect_route_surfaces(context: &ScanContext) -> Vec<RouteSurface> {
    let mut routes: BTreeMap<(PathBuf, HttpMethod, String), RouteSurface> = BTreeMap::new();

    for file in &context.changed_files {
        for hunk in &file.hunks {
            detect_hunk_routes(&file.path, hunk, &mut routes);
        }
    }

    routes.into_values().collect()
}

fn detect_hunk_routes(
    file: &Path,
    hunk: &DiffHunk,
    routes: &mut BTreeMap<(PathBuf, HttpMethod, String), RouteSurface>,
) {
    let before_text = hunk.before_text();

    let after_text = hunk.after_text();

    for route in parse_routes(&before_text) {
        let key = (file.to_path_buf(), route.method, route.path.clone());

        let surface = routes.entry(key).or_insert_with(|| RouteSurface {
            file: file.to_path_buf(),

            method: route.method,
            path: route.path.clone(),

            before: None,
            after: None,

            before_line: None,
            after_line: None,
        });

        surface.before = Some(RouteState::new(route.handlers));

        surface.before_line = Some(hunk.old_start + route.line_offset);
    }

    for route in parse_routes(&after_text) {
        let key = (file.to_path_buf(), route.method, route.path.clone());

        let surface = routes.entry(key).or_insert_with(|| RouteSurface {
            file: file.to_path_buf(),

            method: route.method,
            path: route.path.clone(),

            before: None,
            after: None,

            before_line: None,
            after_line: None,
        });

        surface.after = Some(RouteState::new(route.handlers));

        surface.after_line = Some(hunk.new_start + route.line_offset);
    }
}

fn parse_routes(source: &str) -> Vec<ParsedRoute> {
    let mut routes = Vec::new();

    for captures in ROUTE_PATTERN.captures_iter(source) {
        let Some(full_match) = captures.get(0) else {
            continue;
        };

        let Some(method_match) = captures.get(1) else {
            continue;
        };

        let Some(method) = HttpMethod::parse(method_match.as_str()) else {
            continue;
        };

        /*
         * ROUTE_PATTERN ends exactly after '('.
         */
        let open_paren = full_match.end() - 1;

        let Some(close_paren) = find_matching_paren(source, open_paren) else {
            continue;
        };

        let arguments = &source[open_paren + 1..close_paren];

        let arguments = split_top_level_arguments(arguments);

        let Some(path_argument) = arguments.first() else {
            continue;
        };

        let Some(path) = parse_static_string(path_argument) else {
            /*
             * V0 intentionally ignores
             * dynamically-generated paths.
             */
            continue;
        };

        let handlers = arguments
            .iter()
            .skip(1)
            .map(|argument| argument.trim().to_owned())
            .filter(|handler| !handler.is_empty())
            .collect();

        let line_offset = source[..full_match.start()]
            .bytes()
            .filter(|byte| *byte == b'\n')
            .count();

        routes.push(ParsedRoute {
            method,
            path,
            handlers,
            line_offset,
        });
    }

    routes
}

fn find_matching_paren(source: &str, open_index: usize) -> Option<usize> {
    let mut depth = 0usize;

    let mut quote: Option<char> = None;
    let mut escaped = false;

    for (offset, character) in source[open_index..].char_indices() {
        let index = open_index + offset;

        if let Some(active_quote) = quote {
            if escaped {
                escaped = false;
                continue;
            }

            if character == '\\' {
                escaped = true;
                continue;
            }

            if character == active_quote {
                quote = None;
            }

            continue;
        }

        match character {
            '"' | '\'' | '`' => {
                quote = Some(character);
            }

            '(' => {
                depth += 1;
            }

            ')' => {
                if depth == 0 {
                    return None;
                }

                depth -= 1;

                if depth == 0 {
                    return Some(index);
                }
            }

            _ => {}
        }
    }

    None
}

fn split_top_level_arguments(source: &str) -> Vec<String> {
    let mut arguments = Vec::new();

    let mut current_start = 0usize;

    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut brace_depth = 0usize;

    let mut quote: Option<char> = None;
    let mut escaped = false;

    for (index, character) in source.char_indices() {
        if let Some(active_quote) = quote {
            if escaped {
                escaped = false;
                continue;
            }

            if character == '\\' {
                escaped = true;
                continue;
            }

            if character == active_quote {
                quote = None;
            }

            continue;
        }

        match character {
            '"' | '\'' | '`' => {
                quote = Some(character);
            }

            '(' => {
                paren_depth += 1;
            }

            ')' => {
                paren_depth = paren_depth.saturating_sub(1);
            }

            '[' => {
                bracket_depth += 1;
            }

            ']' => {
                bracket_depth = bracket_depth.saturating_sub(1);
            }

            '{' => {
                brace_depth += 1;
            }

            '}' => {
                brace_depth = brace_depth.saturating_sub(1);
            }

            ',' if paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 => {
                arguments.push(source[current_start..index].trim().to_owned());

                current_start = index + character.len_utf8();
            }

            _ => {}
        }
    }

    let remaining = source[current_start..].trim();

    if !remaining.is_empty() {
        arguments.push(remaining.to_owned());
    }

    arguments
}

fn parse_static_string(value: &str) -> Option<String> {
    let value = value.trim();

    if value.len() < 2 {
        return None;
    }

    let first = value.chars().next()?;

    let last = value.chars().last()?;

    if !matches!(first, '"' | '\'' | '`') {
        return None;
    }

    if first != last {
        return None;
    }

    /*
     * Dynamic template literals are deliberately
     * ignored in the first detector version.
     */
    if first == '`' && value.contains("${") {
        return None;
    }

    Some(value[first.len_utf8()..value.len() - last.len_utf8()].to_owned())
}
