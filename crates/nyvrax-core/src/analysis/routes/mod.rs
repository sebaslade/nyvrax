mod detector;
mod route_surface;

pub use detector::detect_route_surfaces;

pub use route_surface::{HttpMethod, RouteState, RouteSurface};
