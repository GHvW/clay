
use crate::byte_reader::{ ByteReader };
use crate::primitive_readers::{ ReadDouble, DataOps };

pub fn bounding_box_reader(ops: ReadDouble) -> ByteReader<ReadDouble> {
    ByteReader::new(ops, 4)
}

pub struct BoxR {
    byte_reader: ByteReader<ReadDouble>
}

impl BoxR {
    pub fn new(ops: ReadDouble) -> Self {
        Self {
            byte_reader: ByteReader::new(ops, 4)
        }
    }
}

impl DataOps for BoxR {
    type Out = Vec<f64>;

    fn read(&self, start: usize, bytes: &[u8]) -> Option<Self::Out> {
        self.byte_reader.read(start, bytes)
    }

    fn size(&self) -> usize {
        self.byte_reader.size()
    }
}