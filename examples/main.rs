use std::io::prelude::*;
use std::fs::File;
use std::convert::TryInto;

extern crate clay;

// examples in rust http://xion.io/post/code/rust-examples.html

fn main() {
    println!("Hello, world!");

    let mut file_handle = 
        File::open("./examples/shape_file/supercool.shp")
            .expect("Couldn't open file");

    let mut buffer = Vec::new();

    // read the whole file
    file_handle
        .read_to_end(&mut buffer)
        .expect("Error reading bytes into buffer");

    println!("size of shapefile is {} bytes", buffer.len());

    let initial: [u8; 4] = 
        buffer[0..4]
            .try_into()
            .unwrap();

    println!("system endianness is {:?}", clay::endian::determine_system_endianness(initial).unwrap());
}
