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
pub mod shape_readers;

// #![warn(clippy::all)]
// #[wasm_bindgen]
// pub fn read_shapes(bytes: &[u8]) -> Option<ShapeFileData> {

// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
