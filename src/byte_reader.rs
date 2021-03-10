use crate::primitive_readers::DataOps;
use crate::util::gen_intervals;
use crate::shapes::Point;
use crate::primitive_readers::ReadDouble;

#[derive(Copy, Clone, Debug)]
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
}

impl<A: DataOps + Sized> DataOps for ByteReader<A> {
    type Out = Vec<<A as DataOps>::Out>;

    fn read(&self, start: usize, bytes: &[u8]) -> Option<Self::Out> {
        (0..self.read_count)
            .zip(gen_intervals(start, self.read_count, self.ops.size()))
            .map(|(_, i_start)| self.ops.read(i_start, bytes))
            .collect()
    }

    fn size(&self) -> usize {
        self.ops.size() * self.read_count
    }
}
