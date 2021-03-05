use std::io::prelude::*;
use std::fs::File;
// use std::convert::TryInto;

extern crate clay;

// use clay::endian::Endian;
// use clay::play::{ read_main_file_header, read_version_and_shape_type };
use clay::primitive_readers::{ DataOps };
use clay::PrimitiveReaderFactory;

fn main() {
    println!("Hello, world!");

    let mut file_handle = 
        File::open("./examples/shape_file/supercool.shp")
        // File::open("./examples/defaultish/defautish.shp")
            .expect("Couldn't open file");

    let mut buffer = Vec::new();

    // read the whole file
    file_handle
        .read_to_end(&mut buffer)
        .expect("Error reading bytes into buffer");

    let factory = PrimitiveReaderFactory::new();

    let main_file_header_r = &factory.make_main_file_header_reader();

    let shape_reader = &factory.make_polyr();

    println!("main file header: {:?}", main_file_header_r.read(0, &buffer).unwrap());
    println!("main size: {}", main_file_header_r.size());

    let mut offset = 108;
    let mut results = Vec::new();
    // let it = shape_reader.read(offset, &buffer).unwrap();
    // println!("it is {:?}", it.1);
    // while let Some(poly) = shape_reader.read(offset, &buffer) {
    while offset != 454 {
        println!("current offset: {}", offset);
        println!("size: {}", shape_reader.size());
        let poly = shape_reader.read(offset, &buffer).unwrap();
        println!("the poly: {:?}", poly.1);
        results.push(poly.1);
        offset += shape_reader.size() + 8;
        println!("new offset: {}", offset);
    }

    // println!("results: {:?}", results);
}