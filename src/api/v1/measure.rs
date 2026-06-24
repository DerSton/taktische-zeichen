use crate::api::v1::GroundType;
use crate::api::v1::RenderContext;
use svg::node::element::{Group, Path, Polygon};

pub struct MeasureType;

impl GroundType for MeasureType {
    fn render(&self, _context: &RenderContext) -> Group {
        let fill_polygon = Polygon::new()
            .set(
                "points",
                "2.835 11.339 45.354 82.205 87.874 11.339 2.835 11.339",
            )
            .set("fill", "#fff");

        let outline_path = Path::new()
            .set("d", "M46.57,82.934h-2.431L1.619,12.068l1.215-2.146h85.039l1.215,2.146-42.52,70.866ZM5.338,12.756l40.016,66.694L85.371,12.756H5.338Z")
            .set("fill", "#003296");

        Group::new().add(fill_polygon).add(outline_path)
    }
}
