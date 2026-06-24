use crate::api::v1::GroundType;
use crate::api::v1::RenderContext;
use svg::node::element::{Group, Path, Rectangle};

pub struct PersonType;

impl GroundType for PersonType {
    fn render(&self, _context: &RenderContext) -> Group {
        let fill_rect = Rectangle::new()
            .set("x", 15.288)
            .set("y", 15.288)
            .set("width", 60.132)
            .set("height", 60.132)
            .set("transform", "translate(45.354 -18.786) rotate(45)")
            .set("fill", "#fff");

        let outline_path = Path::new()
            .set("d", "M45.354,88.876L1.832,45.354,45.354,1.832l43.522,43.522-43.522,43.522ZM3.837,45.354l41.518,41.518,41.518-41.518L45.354,3.837,3.837,45.354Z");

        Group::new().add(fill_rect).add(outline_path)
    }
}
