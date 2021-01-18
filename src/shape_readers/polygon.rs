use crate::shapes::{ Point, Polygon };
use crate::primitive_readers::{ ReadDouble, DataOps };
use crate::byte_reader::ByteReader;
use crate::shape_readers::point::PointR;

pub struct PolygonR {
    point_reader: PointR
}

impl DataOps for PolygonR {
    type Out = Polygon;

    fn read(&self, start: usize, bytes: &[u8]) -> Option<Self::Out> {
        
    }

    fn size(&self) -> usize {
        self.point_reader.size()
    }
}