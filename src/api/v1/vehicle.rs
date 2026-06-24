use crate::api::v1::GroundType;
use crate::api::v1::RenderContext;
use svg::node::element::{Group, Path};

pub struct VehicleType;

impl GroundType for VehicleType {
    fn render(&self, _context: &RenderContext) -> Group {
        let fill_path = Path::new()
            .set("d", "M45.354,22.677c-17.008,0-31.181-2.582-42.52-6.378v57.402h85.04V16.3c-11.339,3.794-25.512,6.378-42.52,6.378Z")
            .set("fill", "#fff");

        let outline_path = Path::new()
            .set("d", "M88.583,74.41H2.125V15.315l.934.312c12.57,4.208,26.8,6.342,42.295,6.342s29.718-2.134,42.295-6.341l.934-.312v59.095ZM3.543,72.993h83.622V17.279c-12.491,4.053-26.551,6.107-41.811,6.107s-29.328-2.054-41.811-6.107v55.713Z");

        Group::new().add(fill_path).add(outline_path)
    }
}
