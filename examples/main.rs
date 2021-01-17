use std::io::prelude::*;
use std::fs::File;
// use std::convert::TryInto;

extern crate clay;

// examples in rust http://xion.io/post/code/rust-examples.html
// run with cargo run --example main

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

    println!("size of shapefile is {} bytes", &buffer.len());

    // let initial: [u8; 4] = 
    //     buffer[0..4]
    //         .try_into()
    //         .unwrap();

    // println!("file code is {:?}", i32::from_be_bytes(initial));

    let main = clay::play::read_main_file_header(&buffer).unwrap();
    println!("the main file bytes: {:?}", main);

    let versiontype = clay::play::read_version_and_shape_type(&buffer).unwrap();
    println!("version and shapetype is: {:?}", versiontype);

    let bounds = clay::play::read_bounds(&buffer).unwrap();
    println!("bounds are {:?}", bounds);

    // let double = clay::primitive_metadata::DataSize::Double;
    let int = clay::primitive_metadata::DataSize::Int;

    // let little = clay::endian::Endian::Little;
    let big = clay::endian::Endian::Big;

    let main_header_metadata = clay::primitive_metadata::PrimitiveMetadata::new(int, big);

    let main_reader = clay::play::ByteReader::new(main_header_metadata, 7, 0);

    println!("main reader is: {:?}", &main_reader);
    println!("did this work? {:?}", main_reader.read(&buffer).unwrap());
}
