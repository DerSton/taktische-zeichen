use axum::{
    body::Body,
    http::{Response, header},
};
use svg::Document;
use svg::node::element::Circle;

pub async fn generate_svg() -> Response<Body> {
    let circle = Circle::new()
        .set("cx", 50)
        .set("cy", 50)
        .set("r", 40)
        .set("fill", "#003399");

    let document = Document::new().add(circle);
    let svg_content = document.to_string();

    Response::builder()
        .header(header::CONTENT_TYPE, "image/svg+xml")
        .body(Body::from(svg_content))
        .unwrap()
}
