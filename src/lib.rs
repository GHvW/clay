use wasm_bindgen::prelude::*;

pub mod endian;
// pub mod shapes;
pub mod shape_type;
pub mod multi_patch_part_type;
pub mod main_file_header;
pub mod primitive_readers;
pub mod byte_reader;
pub mod util;
pub mod record_header;
pub mod shape_readers;
pub mod shape_data;
// new stuff
pub mod parser;
pub mod shapes;

// use crate::endian::Endian;
// use crate::primitive_readers::{ReadDouble, ReadInt};
// use crate::shape_readers::polygon::{ PolygonRecordR, PolygonStatsR };
// use crate::shape_readers::bounds_box::BoxR;
// use crate::shape_readers::point::PointR;
// use crate::main_file_header::{MainFileHeaderR};
// use crate::record_header::RecordHeaderR;


// #![warn(clippy::all)]


// #[wasm_bindgen]
// pub struct ShapeReaderFactory {
//     big_int: ReadInt,
//     big_double: ReadDouble,
//     little_int: ReadInt,
//     little_double: ReadDouble,
// }

// // #[wasm_bindgen]
// impl ShapeReaderFactory {
//     // #[wasm_bindgen(constructor)]
//     pub fn new() -> Self {
//         Self {
//             big_int: ReadInt::new(Endian::Big),
//             big_double: ReadDouble::new(Endian::Big),
//             little_int: ReadInt::new(Endian::Little),
//             little_double: ReadDouble::new(Endian::Little),
//         }
//     }

//     pub fn make_boxr(&self) -> BoxR {
//         BoxR::new(&self.little_double)
//     }

//     pub fn make_pointr(&self) -> PointR {
//         PointR::new(&self.little_double)
//     }

//     pub fn make_polyr(&self) -> PolygonRecordR {
//         PolygonRecordR::new(
//             RecordHeaderR::new(&self.big_int),
//             PolygonStatsR::new(BoxR::new(&self.little_double), &self.little_int), 
//             &self.little_int, 
//             PointR::new(&self.little_double))
//     }

//     pub fn make_main_file_header_reader(&self) -> MainFileHeaderR {
//         MainFileHeaderR::new(&self.little_int, &self.big_int, &self.little_double)
//     }
// }


#[cfg(test)]
mod tests {

    #[test]
    fn makes_good_mainfileheaderr() {

    }
}
