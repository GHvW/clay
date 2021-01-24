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
use crate::shape_readers::polygon::{ PolygonRecordR, PolygonR, PolygonStatsR };
use crate::shape_readers::bounds_box::BoxR;
use crate::shape_readers::point::PointR;
// #![warn(clippy::all)]
// #[wasm_bindgen]
// pub fn read_shapes(bytes: &[u8]) -> Option<ShapeFileData> {

// }


pub struct PrimitiveReaderFactory {
    big_int: ReadInt,
    big_double: ReadDouble,
    little_int: ReadInt,
    little_double: ReadDouble,
}

impl PrimitiveReaderFactory {
    pub fn new() -> Self {
        Self {
            big_int: ReadInt::new(Endian::Big),
            big_double: ReadDouble::new(Endian::Big),
            little_int: ReadInt::new(Endian::Little),
            little_double: ReadDouble::new(Endian::Little),
        }
    }

    pub fn make_boxr(&self) -> BoxR {
        BoxR::new(&self.big_double)
    }

    pub fn make_pointr(&self) -> PointR {
        PointR::new(&self.big_double)
    }

    pub fn make_polyr(&self) -> PolygonRecordR {
        PolygonRecordR::new(
            &self.big_int, 
            PolygonR::new(
                PolygonStatsR::new(BoxR::new(&self.big_double), &self.big_int), 
                &self.big_int, 
                PointR::new(&self.big_double)))
    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
