use axum::{
    Router,
    body::Body,
    extract::Query,
    http::{Response, header},
    routing::get,
};
use serde::Deserialize;
use svg::Document;
use svg::node::element::Rectangle;

#[derive(Deserialize)]
pub struct SvgParams {
    background: Option<String>,
}

pub fn router() -> Router {
    Router::new().route("/", get(generate_svg))
}

async fn generate_svg(Query(params): Query<SvgParams>) -> Response<Body> {
    let rect = Rectangle::new()
        .set("x", 7)
        .set("y", 21)
        .set("width", 86)
        .set("height", 58)
        .set("fill", "white")
        .set("stroke", "black")
        .set("stroke-width", 2);

    let mut document = Document::new()
        .set("viewBox", (0, 0, 100, 100))
        .set("width", 100)
        .set("height", 100);

    if let Some(color) = params.background {
        let bg_rect = Rectangle::new()
            .set("x", 0)
            .set("y", 0)
            .set("width", 100)
            .set("height", 100)
            .set("fill", color);
        document = document.add(bg_rect);
    }

    document = document.add(rect);

    let svg_content = document.to_string();

    Response::builder()
        .header(header::CONTENT_TYPE, "image/svg+xml")
        .body(Body::from(svg_content))
        .unwrap()
}
