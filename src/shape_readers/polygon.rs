use std::convert::TryInto;

use crate::shapes::{ Point, Polygon };
use crate::primitive_readers::{ ReadInt, ReadDouble, DataOps };
use crate::byte_reader::ByteReader;
use crate::shape_readers::point::PointR;
use crate::shape_readers::bounds_box::BoxR;
use crate::shapes::BoundingBox;


pub struct PolygonStats {
    pub bounds_box: BoundingBox,
    pub parts_count: i32,
    pub points_count: i32 
}

impl PolygonStats {
    pub fn new(bounds_box: BoundingBox, parts_count: i32, points_count: i32) -> Self {
        Self {
            bounds_box,
            parts_count,
            points_count
        }
    }
}


pub struct PolygonStatsR<'a> {
    box_reader: &'a BoxR<'a>,
    int_reader: &'a ReadInt<'a>
}

impl<'a> PolygonStatsR<'a> {
    pub fn new(box_reader: &'a BoxR<'a>, int_reader: &'a ReadInt<'a>) -> Self {
        Self {
            box_reader,
            int_reader
        }
    }
}

impl<'a> DataOps for PolygonStatsR<'a> {
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



pub struct PolygonPointsR<'a> {
    part_reader: ByteReader<'a, ReadInt<'a>>,
    point_reader: ByteReader<'a, PointR<'a>>
}

impl<'a> PolygonPointsR<'a> {
    pub fn new(parts_count: i32, points_count: i32, int_reader: &'a ReadInt, point_reader: &'a PointR) -> Self {
        Self {
            part_reader: ByteReader::new(int_reader, parts_count.try_into().unwrap()),
            point_reader: ByteReader::new(point_reader, points_count.try_into().unwrap()) // TODO better way to handle than unwrap?
        }
    }
}

impl<'a> DataOps for PolygonPointsR<'a> {
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


pub struct PolygonR<'a> {
    stats_reader: PolygonStatsR<'a>,
    int_reader: ReadInt<'a>,
    point_reader: PointR<'a>
}

impl<'a> DataOps for PolygonR<'a> {
    type Out = Polygon;

    fn read(&self, start: usize, bytes: &[u8]) -> Option<Self::Out> {
        let stats = self.stats_reader.read(start, bytes)?;

        let points_reader = 
            PolygonPointsR::new(
                stats.parts_count, 
                stats.points_count, 
                &self.int_reader, 
                &self.point_reader);

        let (parts, points) = points_reader.read(start + self.stats_reader.size(), bytes)?;

        Some(Polygon::new(stats.bounds_box, stats.parts_count, stats.points_count, parts, points))
    }

    fn size(&self) -> usize {
        self.point_reader.size() + self.stats_reader.size()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]

}