use crate::api::v1::GroundType;
use crate::api::v1::RenderContext;
use svg::node::element::{Group, Path};

pub struct AircraftType;

impl GroundType for AircraftType {
    fn render(&self, _context: &RenderContext) -> Group {
        let fill_path = Path::new()
            .set(
                "d",
                "M87.874,65.197H2.835c0-23.483,19.037-42.52,42.52-42.52s42.52,19.037,42.52,42.52Z",
            )
            .set("fill", "#fff");

        let outline_path = Path::new()
            .set("d", "M88.583,65.906H2.126v-.709c0-23.836,19.392-43.229,43.229-43.229s43.229,19.392,43.229,43.229v.709ZM3.549,64.488h83.61c-.379-22.729-18.987-41.102-41.805-41.102S3.928,41.759,3.549,64.488Z");

        Group::new().add(fill_path).add(outline_path)
    }
}
