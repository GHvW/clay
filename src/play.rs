use std::convert::TryInto;

// use crate::endian::{ Endian };
use crate::primitive_readers::{ PrimitiveMetadata, DataOps };
use crate::shapes::Point;


pub fn gen_intervals(start: usize, interval_count: usize, typesize: usize) -> impl Iterator<Item = usize> {
    (0..interval_count).map(move |x| start + (x * typesize))
}

pub fn read_main_file_header(bytes: &[u8]) -> Option<Vec<i32>> {
    (0..7)
        .zip(gen_intervals(0, 7, 4))
        .map(|(_, start)| {
            let end = start + 4;
            bytes[start..end]
                .try_into()
                .map(|int_bytes| {
                    i32::from_be_bytes(int_bytes)
                })
                .ok()
        })
        .collect()
}

pub fn read_version_and_shape_type(bytes: &[u8]) -> Option<Vec<i32>> {
    (0..2)
        .zip(gen_intervals(28, 2, 4))
        .map(|(_, start)| {
            let end = start + 4;
            bytes[start..end]
                .try_into()
                .map(|int_bytes| {
                    i32::from_le_bytes(int_bytes)
                })
                .ok()
        })
        .collect()
}

pub fn read_bounds(bytes: &[u8]) -> Option<Vec<f64>> {
    (0..8)
        .zip(gen_intervals(36, 8, 8))
        .map(|(_, start)| {
            let end = start + 8;
            bytes[start..end]
                .try_into()
                .map(|double_bytes| {
                    f64::from_le_bytes(double_bytes)
                })
                .ok()
        })
        .collect()
}

// pub fn read_point(bytes: &[u8]) -> Option<Point> {
//     (0..2)
//         .zip(gen_intervals())
// }

#[derive(Debug)]
pub struct ByteReader<A: DataOps + Sized> {
    // primitive_metadata: std::marker::PhantomData<A>,
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

    pub fn read(&self, bytes: &[u8]) -> Option<Vec<<A as DataOps>::Out>> {
        println!("");
        (0..self.read_count)
            .zip(gen_intervals(self.start_byte, self.read_count, self.ops.size()))
            .inspect(|(i, start)| println!("index is {}, start is {}", i, start))
            .map(|(_, start)| {
                self.ops
                    .read(start, bytes)
                    .ok()
            })
            .collect()
    }
}

