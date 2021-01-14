use std::io::prelude::*;
use std::fs::File;


fn main() {
    println!("Hello, world!");

    let mut file_handle = 
        File::open("../shape_file/supercool.shp")
            .expect("Couldn't open file");

    let mut buffer = Vec::new();

    // read the whole file
    file_handle
        .read_to_end(&mut buffer)
        .expect("Error reading bytes into buffer");

    println!("size of shapefile is {} bytes", buffer.len());
}
