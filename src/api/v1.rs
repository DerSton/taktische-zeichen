use axum::{
    Router,
    body::Body,
    extract::Query,
    http::{Response, header},
    routing::get,
};
use serde::Deserialize;
use svg::Document;
use svg::node::element::Group;
use svg::node::element::Rectangle;

pub mod aircraft;
pub mod building;
pub mod danger;
pub mod facility;
pub mod formation;
pub mod measure;
pub mod person;
pub mod resource;
pub mod vehicle;
pub mod watercraft;

pub trait GroundType {
    fn render(&self, context: &RenderContext) -> Group;
}

pub struct RenderContext {
    pub background: Option<String>,
}

#[derive(Deserialize)]
pub struct SvgParams {
    pub r#type: Option<String>,
    pub background: Option<String>,
}

pub fn router() -> Router {
    Router::new().route("/", get(generate_svg))
}

pub fn get_type(name: &str) -> Box<dyn GroundType> {
    match name.to_lowercase().as_str() {
        "formation" => Box::new(formation::FormationType),
        "person" => Box::new(person::PersonType),
        "vehicle" | "land_vehicle" | "landfahrzeug" => Box::new(vehicle::VehicleType),
        "aircraft" | "luftfahrzeug" => Box::new(aircraft::AircraftType),
        "watercraft" | "wasserfahrzeug" => Box::new(watercraft::WatercraftType),
        "facility" | "einrichtung" | "funktionsstelle" => Box::new(facility::FacilityType),
        "building" | "gebäude" => Box::new(building::BuildingType),
        "resource" | "ressource" => Box::new(resource::ResourceType),
        "measure" | "maßnahme" => Box::new(measure::MeasureType),
        "danger" | "gefahr" => Box::new(danger::DangerType),
        _ => Box::new(formation::FormationType),
    }
}

async fn generate_svg(Query(params): Query<SvgParams>) -> Response<Body> {
    let context = RenderContext {
        background: params.background.clone(),
    };

    let mut document = Document::new()
        .set("viewBox", (0, 0, 90.709, 90.709))
        .set("width", 100)
        .set("height", 100);

    if let Some(color) = &context.background {
        let bg_rect = Rectangle::new()
            .set("x", 0)
            .set("y", 0)
            .set("width", 90.709)
            .set("height", 90.709)
            .set("fill", color.as_str());
        document = document.add(bg_rect);
    }

    let type_name = params.r#type.as_deref().unwrap_or("formation");
    let ground_type = get_type(type_name);
    let type_group = ground_type.render(&context);

    document = document.add(type_group);

    let svg_content = document.to_string();

    Response::builder()
        .header(header::CONTENT_TYPE, "image/svg+xml")
        .body(Body::from(svg_content))
        .unwrap()
}
