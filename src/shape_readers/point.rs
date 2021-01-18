use crate::shapes::Point;
use crate::primitive_readers::{ ReadDouble, DataOps };
use crate::byte_reader::ByteReader;

pub struct PointR {
    byte_reader: ByteReader<ReadDouble>
}

impl PointR {
    pub fn new(byte_reader: ByteReader<ReadDouble>) -> Self {
        Self { byte_reader }
    }
}

impl DataOps for PointR {
    type Out = Point;

    fn read(&self, start: usize, bytes: &[u8]) -> Option<Self::Out> {
        self.byte_reader
            .read(start, bytes)
            .map(|vec| {
                Point::new(vec[0], vec[1])
            })
    }

    fn size(&self) -> usize {
        self.byte_reader.size()
    }
}