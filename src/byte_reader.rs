use crate::primitive_readers::DataOps;
use crate::util::gen_intervals;

#[derive(Debug)]
pub struct ByteReader<A: DataOps + Sized> {
    ops: A,
    read_count: usize,
    start_byte: usize,
}

impl<A: DataOps + Sized> ByteReader<A> {
    pub fn new(ops: A, read_count: usize, start_byte: usize) -> Self {
        Self {
            ops,
            read_count,
            start_byte,
        }
    }
}

impl<> DataOps for ByteReader {
    fn read(&self, bytes: &[u8]) -> Option<Vec<<A as DataOps>::Out>> {
        println!("");
        (0..self.read_count)
            .zip(gen_intervals(self.start_byte, self.read_count, self.ops.size()))
            .map(|(_, start)| {
                self.ops
                    .read(start, bytes)
                    .ok()
            })
            .collect()
    }
}