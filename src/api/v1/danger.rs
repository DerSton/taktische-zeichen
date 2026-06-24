use crate::api::v1::GroundType;
use crate::api::v1::RenderContext;
use svg::node::element::{Group, Path, Polygon};

pub struct DangerType;

impl GroundType for DangerType {
    fn render(&self, _context: &RenderContext) -> Group {
        let fill_polygon = Polygon::new()
            .set(
                "points",
                "2.835 79.37 45.354 8.504 87.874 79.37 2.835 79.37",
            )
            .set("fill", "#fff");

        let outline_path = Path::new()
            .set("d", "M87.874,80.787H2.835s-1.215-2.147-1.215-2.147L44.139,7.775h2.431l42.52,70.866-1.215,2.146ZM5.338,77.953h80.033S45.354,11.259,45.354,11.259L5.338,77.953Z")
            .set("fill", "#fa1919");

        Group::new().add(fill_polygon).add(outline_path)
    }
}
