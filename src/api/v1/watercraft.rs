use crate::api::v1::GroundType;
use crate::api::v1::RenderContext;
use svg::node::element::{Group, Path};

pub struct WatercraftType;

impl GroundType for WatercraftType {
    fn render(&self, _context: &RenderContext) -> Group {
        let fill_path = Path::new()
            .set(
                "d",
                "M2.835,25.512h85.039c0,23.483-19.037,42.52-42.52,42.52S2.835,48.995,2.835,25.512Z",
            )
            .set("fill", "#fff");

        let outline_path = Path::new()
            .set("d", "M45.354,68.74C21.518,68.74,2.126,49.348,2.126,25.512v-.709h86.457v.709c0,23.836-19.392,43.229-43.229,43.229ZM3.549,26.221c.379,22.729,18.987,41.102,41.805,41.102s41.426-18.373,41.805-41.102H3.549Z");

        Group::new().add(fill_path).add(outline_path)
    }
}
