use std::io::prelude::*;
use std::fs::File;
// use std::convert::TryInto;

extern crate clay;

use clay::endian::Endian;
use clay::primitive_readers::DataOps;
// examples in rust http://xion.io/post/code/rust-examples.html
// run with cargo run --example main

fn main() {
    println!("Hello, world!");

    let mut file_handle = 
        // File::open("./examples/shape_file/supercool.shp")
        File::open("./examples/defaultish/defautish.shp")
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


    // let little = clay::endian::Endian::Little;
    // let big = clay::endian::Endian::Big;

    let main_header_metadata = clay::primitive_readers::ReadInt::new(Endian::Big);

    let main_reader = clay::byte_reader::ByteReader::new(&main_header_metadata, 7);

    println!("main reader is: {:?}", &main_reader);
    println!("did this work? {:?}", main_reader.read(0, &buffer).unwrap());

    // let little = clay::endian::Endian::Little;

    let little_int = clay::primitive_readers::ReadInt::new(Endian::Little);

    let little_double = clay::primitive_readers::ReadDouble::new(Endian::Little);

    let box_r = clay::shape_readers::bounds_box::BoxR::new(&little_double);

    let point_r = clay::shape_readers::point::PointR::new(&little_double);

    let stats = clay::shape_readers::polygon::PolygonStatsR::new(box_r, &little_int);

    let polygon_r = clay::shape_readers::polygon::PolygonR::new(stats, &little_int, point_r);

    let polygon_record_r = clay::shape_readers::polygon::PolygonRecordR::new(&little_int, polygon_r);

    let (shape_kind, poly) = polygon_record_r.read(108, &buffer).unwrap();

    println!("shape type is {}", shape_kind);
    println!("polygons: {:?}", poly);
}
