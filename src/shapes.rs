#[derive(Debug)]
pub struct BoundingBox {
    pub x_min: f64,
    pub y_min: f64,
    pub x_max: f64,
    pub y_max: f64
}

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
pub struct MultiPoint {
    pub bounding_box: BoundingBox,
    pub points_count: i32,
    pub points: Vec<Point>
}

pub struct PolyLine {
    pub bounding_box: BoundingBox,
    pub parts_count: i32,
    pub points_count: i32,
    pub parts: Vec<i32>,
    pub points: Vec<Point>
}

pub struct Polygon {
    pub bounding_box: BoundingBox,
    pub parts_count: i32,
    pub points_count: i32,
    pub parts: Vec<i32>,
    pub points: Vec<Point>
}

impl Polygon {
    pub fn new() -> Self {
        // points need to come together at one point
        unimplemented!();
    }
}

pub struct PointM {
    pub x: f64,
    pub y: f64,
    pub measure: f64
}


pub struct MultiPointM {
    pub bounding_box: BoundingBox,
    pub points_count: i32,
    pub points: Vec<Point>,
    pub bounding_measure_range: Vec<f64>,
    pub measures: Vec<f64>
}

pub struct PolyLineM {
    pub bounding_box: BoundingBox,
    pub parts_count: i32,
    pub points_count: i32,
    pub parts: Vec<i32>,
    pub points: Vec<Point>,
    pub bounding_measure_range: Vec<f64>,
    pub measures: Vec<f64>
}

pub struct PolygonM {
    pub bounding_box: BoundingBox,
    pub parts_count: i32,
    pub points_count: i32,
    pub parts: Vec<i32>,
    pub points: Vec<Point>,
    pub bounding_measure_range: Vec<f64>,
    pub measures: Vec<f64>
}

pub struct PointZ {
    pub x: f64,
    pub y: f64,
    pub measure: f64
}

pub struct MultiPointZ {
    pub bounding_box: BoundingBox,
    pub points_count: i32,
    pub points: Vec<Point>,
    pub bounding_z_range: Vec<f64>,
    pub z_values: Vec<f64>,
    pub bounding_measure_range: Vec<f64>,
    pub measures: Vec<f64>
}


pub struct PolyLineZ {
    pub bounding_box: BoundingBox,
    pub parts_count: i32,
    pub points_count: i32,
    pub parts: Vec<i32>,
    pub points: Vec<Point>,
    pub bounding_z_range: Vec<f64>,
    pub z_values: Vec<f64>,
    pub bounding_measure_range: Vec<f64>,
    pub measures: Vec<f64>
}

pub struct PolygonZ {
    pub bounding_box: BoundingBox,
    pub parts_count: i32,
    pub points_count: i32,
    pub parts: Vec<i32>,
    pub points: Vec<Point>,
    pub bounding_z_range: Vec<f64>,
    pub z_values: Vec<f64>,
    pub bounding_measure_range: Vec<f64>,
    pub measures: Vec<f64>
}


pub struct MultiPatch {
    pub bounding_box: BoundingBox,
    pub parts_count: i32,
    pub points_count: i32,
    pub parts: Vec<i32>,
    pub part_types: Vec<i32>,
    pub points: Vec<Point>,
    pub bounding_z_range: Vec<f64>,
    pub z_values: Vec<f64>,
    pub bounding_measure_range: Vec<f64>,
    pub measures: Vec<f64>
}