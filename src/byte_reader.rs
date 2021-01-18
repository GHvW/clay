use crate::primitive_readers::DataOps;
use crate::util::gen_intervals;
use crate::shapes::Point;
use crate::primitive_readers::ReadDouble;

#[derive(Debug)]
pub struct ByteReader<A: DataOps + Sized> {
    ops: A,
    read_count: usize,
    // start_byte: usize,
}

impl<A: DataOps + Sized> ByteReader<A> {
    // pub fn new(ops: A, read_count: usize, start_byte: usize) -> Self {
    pub fn new (ops: A, read_count: usize) -> Self {
        Self {
            ops,
            read_count,
            // start_byte,
        }
    }

    // pub fn read(&self, bytes: &[u8]) -> Option<Vec<<A as DataOps>::Out>> {
    //     println!("");
    //     (0..self.read_count)
    //         .zip(gen_intervals(self.start_byte, self.read_count, self.ops.size()))
    //         .map(|(_, start)| {
    //             self.ops
    //                 .read(start, bytes)
    //                 .ok()
    //         })
    //         .collect()
    // }
}

impl<A: DataOps + Sized> DataOps for ByteReader<A> {
    type Out = Vec<<A as DataOps>::Out>;

    fn read(&self, start: usize, bytes: &[u8]) -> Option<Self::Out> {
        (0..self.read_count)
            .zip(gen_intervals(start, self.read_count, self.ops.size()))
            .map(|(_, i_start)| {
                self.ops
                    .read(i_start, bytes)
                    // .ok()
            })
            .collect()
    }

    fn size(&self) -> usize {
        self.ops.size() * self.read_count
    }
}



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