// use std::convert::TryInto;
// use crate::endian::Endian;
// use crate::parser::Parser;

// #[derive(Debug)]
// pub struct Integer {
//     endian: Endian
// }

// impl Integer {
//     pub fn new(endian: Endian) -> Self {
//         Self { endian }
//     }
// }

// impl Parser for Integer {
//     type Out = i32;

//     fn call(&self, input: &[u8]) -> Option<(Self::Out, &[u8])> {

//     }
// }

// pub trait DataOps {
//     type Out;

//     fn read(&self, start: usize, bytes: &[u8]) -> Option<Self::Out>;
//     fn size(&self) -> usize;
// }


// #[derive(Debug)]
// pub struct ReadInt {
//     endian: Endian
// }

// impl ReadInt {
//     pub fn new(endian: Endian) -> Self {
//         Self { endian }
//     }
// }

// impl DataOps for ReadInt {
//     type Out = i32;

//     fn read(&self, start: usize, bytes: &[u8]) -> Option<Self::Out> {
//         bytes.get(start..(start + 4))
//             ?.try_into()
//             .map(|int_bytes| {
//                 self.endian.convert_int32(int_bytes)
//             })
//             .ok()
//     }

//     fn size(&self) -> usize {
//         // self.data_size.size()
//         4
//     }
// }


// #[derive(Debug)]
// pub struct ReadDouble {
//     endian: Endian
// }

// impl ReadDouble {
//     pub fn new(endian: Endian) -> Self { 
//         Self { endian } 
//     }
// }

// impl DataOps for ReadDouble {
//     type Out = f64;

//     fn read(&self, start: usize, bytes: &[u8]) -> Option<Self::Out> {
//         bytes.get(start..(start + 8)) 
//             ?.try_into()
//             .map(|double_bytes| {
//                 self.endian.convert_f64(double_bytes)
//             })
//             .ok() 
//     }

//     fn size(&self) -> usize {
//         // self.data_size.size()
//         8
//     }
// }
