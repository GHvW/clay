// new stuff
pub mod bounding_box;
// end new stuff

// #[derive(Debug, PartialEq)]
// pub struct BoundingBox {
//     pub x_min: f64,
//     pub y_min: f64,
//     pub x_max: f64,
//     pub y_max: f64
// }

// impl BoundingBox {
//     pub fn new(x_min: f64, y_min: f64, x_max: f64, y_max: f64) -> Self {
//         Self {
//             x_min,
//             y_min,
//             x_max,
//             y_max
//         }
//     }
// }

// #[derive(Debug, PartialEq)]
// pub struct Point {
//     pub x: f64,
//     pub y: f64
// }

// impl Point {
//     pub fn new(x: f64, y: f64) -> Self {
//         Self { x, y }
//     }
// }


// #[derive(Debug, PartialEq)]
// pub struct MultiPoint {
//     pub bounding_box: BoundingBox,
//     pub points_count: i32,
//     pub points: Vec<Point>
// }


// #[derive(Debug, PartialEq)]
// pub struct PolyLine {
//     pub bounding_box: BoundingBox,
//     pub parts_count: i32,
//     pub points_count: i32,
//     pub parts: Vec<i32>,
//     pub points: Vec<Point>
// }

// #[derive(Debug, PartialEq)]
// pub struct Polygon {
//     pub bounding_box: BoundingBox,
//     pub parts_count: i32,
//     pub points_count: i32,
//     pub parts: Vec<i32>,
//     pub points: Vec<Point>
// }

// impl Polygon {
//     pub fn new(bounding_box: BoundingBox, parts_count: i32, points_count: i32, parts: Vec<i32>, points: Vec<Point>) -> Self {
//         // points need to come together at one point
//         Self {
//             bounding_box,
//             parts_count,
//             points_count,
//             parts,
//             points
//         }
//     }
// }

// pub struct PointM {
//     pub x: f64,
//     pub y: f64,
//     pub measure: f64
// }


// pub struct MultiPointM {
//     pub bounding_box: BoundingBox,
//     pub points_count: i32,
//     pub points: Vec<Point>,
//     pub bounding_measure_range: Vec<f64>,
//     pub measures: Vec<f64>
// }

// pub struct PolyLineM {
//     pub bounding_box: BoundingBox,
//     pub parts_count: i32,
//     pub points_count: i32,
//     pub parts: Vec<i32>,
//     pub points: Vec<Point>,
//     pub bounding_measure_range: Vec<f64>,
//     pub measures: Vec<f64>
// }

// pub struct PolygonM {
//     pub bounding_box: BoundingBox,
//     pub parts_count: i32,
//     pub points_count: i32,
//     pub parts: Vec<i32>,
//     pub points: Vec<Point>,
//     pub bounding_measure_range: Vec<f64>,
//     pub measures: Vec<f64>
// }

// pub struct PointZ {
//     pub x: f64,
//     pub y: f64,
//     pub measure: f64
// }

// pub struct MultiPointZ {
//     pub bounding_box: BoundingBox,
//     pub points_count: i32,
//     pub points: Vec<Point>,
//     pub bounding_z_range: Vec<f64>,
//     pub z_values: Vec<f64>,
//     pub bounding_measure_range: Vec<f64>,
//     pub measures: Vec<f64>
// }


// pub struct PolyLineZ {
//     pub bounding_box: BoundingBox,
//     pub parts_count: i32,
//     pub points_count: i32,
//     pub parts: Vec<i32>,
//     pub points: Vec<Point>,
//     pub bounding_z_range: Vec<f64>,
//     pub z_values: Vec<f64>,
//     pub bounding_measure_range: Vec<f64>,
//     pub measures: Vec<f64>
// }

// pub struct PolygonZ {
//     pub bounding_box: BoundingBox,
//     pub parts_count: i32,
//     pub points_count: i32,
//     pub parts: Vec<i32>,
//     pub points: Vec<Point>,
//     pub bounding_z_range: Vec<f64>,
//     pub z_values: Vec<f64>,
//     pub bounding_measure_range: Vec<f64>,
//     pub measures: Vec<f64>
// }


// pub struct MultiPatch {
//     pub bounding_box: BoundingBox,
//     pub parts_count: i32,
//     pub points_count: i32,
//     pub parts: Vec<i32>,
//     pub part_types: Vec<i32>,
//     pub points: Vec<Point>,
//     pub bounding_z_range: Vec<f64>,
//     pub z_values: Vec<f64>,
//     pub bounding_measure_range: Vec<f64>,
//     pub measures: Vec<f64>
// }