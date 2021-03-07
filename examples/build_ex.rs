use std::io::prelude::*;
use std::fs::File;

extern crate clay;

use clay::primitive_readers::{ DataOps };
use clay::ShapeReaderFactory;

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

    let factory = ShapeReaderFactory::new();

    let main_file_header_r = &factory.make_main_file_header_reader();

    let shape_reader = &factory.make_polyr();

    println!("main file header: {:?}", main_file_header_r.read(0, &buffer).unwrap());
    println!("main size: {}", main_file_header_r.size());

    let mut offset = 100;
    let mut results = Vec::new();

    while let Some(poly) = shape_reader.read_record(offset, &buffer) {
        println!("current offset: {}", offset);
        println!("the poly: {:?}", poly.polygon);
        offset += poly.size;
        results.push(poly);
        println!("new offset: {}", offset);
    }

    println!("results count: {:?}", results.len());
}