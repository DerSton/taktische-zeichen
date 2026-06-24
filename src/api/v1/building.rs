use crate::api::v1::GroundType;
use crate::api::v1::RenderContext;
use svg::node::element::{Group, Path, Polygon};

pub struct BuildingType;

impl GroundType for BuildingType {
    fn render(&self, _context: &RenderContext) -> Group {
        let fill_polygon = Polygon::new()
            .set(
                "points",
                "45.354 8.504 2.835 28.346 2.835 73.701 87.874 73.701 87.874 28.346 45.354 8.504",
            )
            .set("fill", "#fff");

        let outline_path = Path::new()
            .set("d", "M88.583,74.41H2.126V27.895L45.354,7.722l43.229,20.173v46.514ZM3.544,72.992h83.622V29.764H3.544v43.228ZM4.511,28.346h81.687L45.354,9.286,4.511,28.346Z");

        Group::new().add(fill_polygon).add(outline_path)
    }
}
