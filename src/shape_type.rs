
pub enum ShapeType {
    Null,
    Point,
    PolyLine,
    Polygon,
    MultiPoint,
    PointZ,
    PolyLineZ,
    PolygonZ,
    MultiPointZ,
    PointM,
    PolyLineM,
    PolygonM,
    MultiPointM,
    MultiPatch
}

pub fn resolve_shape_type(n: i32) -> Option<ShapeType> {
    match n {
        0 => Some(ShapeType::Null),
        1 => Some(ShapeType::Point),
        3 => Some(ShapeType::PolyLine),
        5 => Some(ShapeType::Polygon),
        8 => Some(ShapeType::MultiPoint),
        11 => Some(ShapeType::PointZ),
        13 => Some(ShapeType::PolyLineZ),
        15 => Some(ShapeType::PolygonZ),
        18 => Some(ShapeType::MultiPointZ),
        21 => Some(ShapeType::PointM),
        23 => Some(ShapeType::MultiPointM),
        25 => Some(ShapeType::PolygonM),
        28 => Some(ShapeType::MultiPointM),
        31 => Some(ShapeType::MultiPatch),
        _ => None
    }
}