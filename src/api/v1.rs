use axum::{
    Router,
    body::Body,
    http::{Response, header},
    routing::get,
};
use svg::Document;
use svg::node::element::Rectangle;

pub fn router() -> Router {
    Router::new().route("/", get(generate_svg))
}

async fn generate_svg() -> Response<Body> {
    let rect = Rectangle::new()
        .set("x", 7)
        .set("y", 21)
        .set("width", 86)
        .set("height", 58)
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 2);

    let document = Document::new()
        .set("viewBox", (0, 0, 100, 100))
        .set("width", 100)
        .set("height", 100)
        .add(rect);

    let svg_content = document.to_string();

    Response::builder()
        .header(header::CONTENT_TYPE, "image/svg+xml")
        .body(Body::from(svg_content))
        .unwrap()
}
