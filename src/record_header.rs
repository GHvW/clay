use crate::primitive_readers::{ ReadInt, DataOps };

#[derive(Debug, Eq, PartialEq)]
pub struct RecordHeader {
    pub record_number: i32,
    pub content_length: i32
}

impl RecordHeader {
    pub fn new(record_number: i32, content_length: i32) -> Self {
        Self {
            record_number,
            content_length
        }
    }
}


pub struct RecordHeaderR {
    int_reader: ReadInt
}

impl RecordHeaderR {
    pub fn new(int_reader: ReadInt) -> Self {
        Self { int_reader }
    }
}

impl DataOps for RecordHeaderR {
    type Out = RecordHeader;

    fn read(&self, start: usize, bytes: &[u8]) -> Option<Self::Out> {
        let record_num = self.int_reader.read(start, bytes)?;
        let content_len = self.int_reader.read(start + self.int_reader.size(), bytes)?;

        Some(RecordHeader::new(record_num, content_len))
    }

    fn size(&self) -> usize {
        self.int_reader.size() + self.int_reader.size()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::primitive_readers::ReadDouble;
    use crate::endian::Endian;

    #[test]
    fn read_record_header() {
        let rec_num_bytes = [0b00000000, 0b00000000, 0b00000000, 0b00000101];
        let content_length_bytes = [0b00000000, 0b00000000, 0b00000000, 0b11001000];
        let bytes = [rec_num_bytes, content_length_bytes].concat();

        let int_reader = ReadInt::new(Endian::Big);

        let reader = RecordHeaderR::new(int_reader);

        // act
        let actual = reader.read(0, &bytes).unwrap();
        
        let expected = RecordHeader::new(5, 200);
        // assert
        assert_eq!(expected, actual);
    }
}
