use crate::api::v1::GroundType;
use crate::api::v1::RenderContext;
use svg::node::element::{Group, Path, Rectangle};

pub struct ResourceType;

impl GroundType for ResourceType {
    fn render(&self, _context: &RenderContext) -> Group {
        let fill_rect = Rectangle::new()
            .set("x", 11.339)
            .set("y", 11.339)
            .set("width", 68.031)
            .set("height", 68.031)
            .set("fill", "#fff");

        let outline_path = Path::new().set(
            "d",
            "M80.079,80.079H10.63V10.63h69.449v69.449ZM12.047,78.661h66.614V12.047H12.047v66.614Z",
        );

        Group::new().add(fill_rect).add(outline_path)
    }
}
