use crate::api::v1::GroundType;
use crate::api::v1::RenderContext;
use svg::node::element::{Circle, Group, Path};

pub struct FacilityType;

impl GroundType for FacilityType {
    fn render(&self, _context: &RenderContext) -> Group {
        let fill_circle = Circle::new()
            .set("cx", 45.354)
            .set("cy", 45.354)
            .set("r", 39.678)
            .set("fill", "#fff");

        let outline_path = Path::new()
            .set("d", "M45.354,85.741c-22.269,0-40.386-18.117-40.386-40.386S23.085,4.967,45.354,4.967s40.387,18.117,40.387,40.387-18.117,40.386-40.387,40.386ZM45.354,6.385C23.867,6.385,6.385,23.866,6.385,45.354s17.481,38.969,38.969,38.969,38.969-17.481,38.969-38.969S66.842,6.385,45.354,6.385Z");

        Group::new().add(fill_circle).add(outline_path)
    }
}
