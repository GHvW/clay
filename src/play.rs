use std::convert::TryInto;

// use crate::endian::{ Endian };
use crate::primitive_readers::{ DataOps };
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


