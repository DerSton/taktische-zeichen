use axum::{Router, routing::get};

mod svg;

pub fn router() -> Router {
    Router::new().route("/svg", get(svg::generate_svg))
}
