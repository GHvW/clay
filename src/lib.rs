// use wasm_bindgen::prelude::*;

pub mod endian;
pub mod shape_type;
pub mod shapes;
pub mod multi_patch_part_type;
pub mod main_file_header;
pub mod play;
pub mod primitive_readers;
pub mod byte_reader;
pub mod util;
pub mod record_header;
pub mod shape_readers;
pub mod shape_data;

use crate::endian::Endian;
use crate::primitive_readers::{ReadDouble, ReadInt};
// #![warn(clippy::all)]
// #[wasm_bindgen]
// pub fn read_shapes(bytes: &[u8]) -> Option<ShapeFileData> {

// }


pub struct ShapeFileReader {
    big_int: ReadInt,
    big_double: ReadDouble,
    little_int: ReadInt,
    little_double: ReadDouble
}

impl ShapeFileReader {
    pub fn new() -> Self {
        Self {
            big_int: ReadInt::new(Endian::Big),
            big_double: ReadDouble::new(Endian::Big),
            little_int: ReadInt::new(Endian::Little),
            little_double: ReadDouble::new(Endian::Little)
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
