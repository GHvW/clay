use crate::shapes::{ Point, Polygon };
use crate::primitive_readers::{ ReadInt, ReadDouble, DataOps };
use crate::byte_reader::ByteReader;
use crate::shape_readers::point::PointR;
use crate::shape_readers::bounds_box::BoxR;
use crate::shapes::BoundingBox;


pub struct PolygonStats {
    pub bounds_box: Vec<f64>,
    pub parts_count: i32,
    pub points_count: i32 
}

impl PolygonStats {
    pub fn new(bounds_box: Vec<f64>, parts_count: i32, points_count: i32) -> Self {
        Self {
            bounds_box,
            parts_count,
            points_count
        }
    }
}


pub struct PolygonStatsR {
    box_reader: BoxR,
    int_reader: ReadInt
}

impl PolygonStatsR {
    pub fn new(box_reader: BoxR, int_reader: ReadInt) -> Self {
        Self {
            box_reader,
            int_reader
        }
    }
}

impl DataOps for PolygonStatsR {
    type Out = PolygonStats;

    fn read(&self, start: usize, bytes: &[u8]) -> Option<Self::Out> {
        let box_size = self.box_reader.size();
        let b = self.box_reader.read(start, bytes)?;
        let f = self.int_reader.read(start + box_size, bytes)?;
        let s = self.int_reader.read(start + box_size + 4, bytes)?;

        Some(PolygonStats::new(b, f, s))
    }

    fn size(&self) -> usize {
        self.box_reader.size() + 8 // two Ints = 8 bytes
    }
}



pub struct PolygonPointsR {
    part_reader: ByteReader<ReadInt>,
    point_reader: ByteReader<PointR>
}

impl PolygonPointsR {
    pub fn new(parts_count: i32, points_count: i32, int_reader: ReadInt, point_reader: PointR) -> Self {
        Self {
            part_reader: ByteReader::new(int_reader, parts_count),
            point_reader: ByteReader::new(point_reader, points_count)
        }
    }
}

impl DataOps for PolygonPointsR {
    type Out = (Vec<i32>, Vec<Point>);

    fn read(&self, start: usize, bytes: &[u8]) -> Option<Self::Out> {
        let parts = self.part_reader.read(start, bytes)?;
        let points = self.point_reader.read(start + self.part_reader.size(), bytes)?;

        Some((parts, points))
    }

    fn size(&self) -> usize {
        self.point_reader.size() + self.part_reader.size()
    }
}


pub struct PolygonR {
    stats_reader: PolygonStatsR,
    int_reader: ReadInt,
    point_reader: PointR
}

impl DataOps for PolygonR {
    type Out = Polygon;

    fn read(&self, start: usize, bytes: &[u8]) -> Option<Self::Out> {
        let stats = self.stats_reader.read(start, bytes)?;
        let points_reader = PolygonPointsR::new(stats.parts_count, stats.points_count, self.int_reader, self.point_reader);
    }

    fn size(&self) -> usize {
        self.points_reader.size() + self.stats_reader.size()
    }
}