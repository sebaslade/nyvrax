use std::{
    error::Error,
    io::{self, Write},
};

use nyvrax_core::{
    Engine, ScanContext, Verdict,
    git::{load_changed_diff, repository_root},
};

use nyvrax_reporters::{JsonReporter, Reporter, SarifReporter, TerminalReporter};

use crate::cli::OutputFormat;

pub fn execute(
    changed: bool,
    base: Option<String>,
    format: OutputFormat,
) -> Result<i32, Box<dyn Error>> {
    if !changed {
        return Err(
            "repository-wide scanning is not implemented yet; use `nyvrax scan --changed`".into(),
        );
    }

    let current_dir = std::env::current_dir()?;

    let root = repository_root(&current_dir)?;

    let diff = load_changed_diff(&root, base.as_deref())?;

    let context = ScanContext::changed(root, diff.files);

    let engine = Engine::default();

    let result = engine.scan(&context)?;

    let output = match format {
        OutputFormat::Terminal => TerminalReporter.render(&result)?,

        OutputFormat::Json => JsonReporter.render(&result)?,

        OutputFormat::Sarif => SarifReporter.render(&result)?,
    };

    let mut stdout = io::stdout().lock();

    writeln!(stdout, "{output}")?;

    Ok(match result.verdict {
        Verdict::Block => 1,
        Verdict::Pass | Verdict::Warn => 0,
    })
}
