use std::convert::TryInto;

use crate::endian::{ Endian, DataSize, DataOps, BytesReader };
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


pub struct Reader {
    data_ops: DataOps,
    read_count: usize,
    start_byte: usize,
}

impl Reader {
    pub fn new(data_ops: DataOps, read_count: usize, start_byte: usize) -> Self {
        Self {
            data_ops,
            read_count,
            start_byte,
        }
    }

    pub fn read(&self, bytes: &[u8]) -> Option<Vec<f64>> {
        (0..self.read_count)
            .zip(gen_intervals(self.start_byte, self.read_count, self.data_ops.data_size.size()))
            .map(|(_, start)| {
                self.data_ops
                    .read(start, bytes)
                    .ok()
            })
            .collect()
    }
}


pub struct DoubleReader {
    read_count: usize,
    start_byte: usize,
    endian: Endian
}

impl DoubleReader {
    pub fn new(read_count: usize, start_byte: usize, endian: Endian) -> Self {
        Self {
            read_count,
            start_byte,
            endian
        }
    }

    pub fn read(&self, bytes: &[u8]) -> Option<Vec<f64>> {
        (0..self.read_count)
            .zip(gen_intervals(self.start_byte, self.read_count, 8))
            .map(|(_, start)| {
                self.endian
                    .convert_double(start, bytes)
                    .ok()
            })
            .collect()
    }
}
// pub fn read_polygon() -> Vec<>