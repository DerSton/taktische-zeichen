use axum::{
    Router,
    body::Body,
    http::{Response, header},
    routing::get,
};
use svg::Document;
use svg::node::element::Circle;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(generate_svg));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn generate_svg() -> Response<Body> {
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
