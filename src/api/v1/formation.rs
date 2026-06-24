use crate::api::v1::GroundType;
use crate::api::v1::RenderContext;
use svg::node::element::{Group, Path, Rectangle};

pub struct FormationType;

impl GroundType for FormationType {
    fn render(&self, _context: &RenderContext) -> Group {
        let fill_rect = Rectangle::new()
            .set("x", 2.835)
            .set("y", 17.008)
            .set("width", 85.04)
            .set("height", 56.693)
            .set("fill", "#fff");

        let outline_path = Path::new().set(
            "d",
            "M88.583,74.41H2.126V16.299h86.457v58.11ZM3.544,72.992h83.622V17.717H3.544v55.275Z",
        );

        Group::new().add(fill_rect).add(outline_path)
    }
}
