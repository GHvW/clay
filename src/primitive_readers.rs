use std::convert::TryInto;

use crate::endian::Endian;

#[derive(Debug)]
pub enum DataSize {
    Int,
    Double
}

impl DataSize {
    pub fn size(&self) -> usize {
        match self {
            DataSize::Int => 4,
            DataSize::Double => 8
        }
    }
}



pub trait DataOps {
    type Out;

    // fn read(&self, start: usize, bytes: &[u8]) -> Result<Self::Out, TryFromSliceError>;
    fn read(&self, start: usize, bytes: &[u8]) -> Option<Self::Out>;
    fn size(&self) -> usize;
}

// #[derive(Debug)]
// pub struct PrimitiveMetadata {
//     pub data_size: DataSize,
//     pub endian: Endian,
// }

// impl<A> PrimitiveMetadata {
//     pub fn new(data_size: DataSize, endian: Endian) -> Self {
//     // pub fn new(data_size: DataSize, endian: Endian, phantom: std::marker::PhantomData<A>) -> Self {
//         Self {
//             data_size,
//             endian,
//             // phantom
//         }
//     }
// }

#[derive(Debug)]
pub struct ReadInt<'a> {
    endian: &'a Endian
}

impl<'a> ReadInt<'a> {
    pub fn new(endian: &'a Endian) -> Self {
        Self { endian }
    }
}

// impl MetadataOps<i32> for PrimitiveMetadata<i32> {
impl<'a> DataOps for ReadInt<'a> {
    type Out = i32;

    // fn read(&self, start: usize, bytes: &[u8]) -> Result<Self::Out, TryFromSliceError> {
    fn read(&self, start: usize, bytes: &[u8]) -> Option<Self::Out> {
        bytes[start..(start + 4)] 
            .try_into()
            .map(|int_bytes| {
                self.endian.convert_int32(int_bytes)
            })
            .ok()
    }

    fn size(&self) -> usize {
        // self.data_size.size()
        4
    }
}


#[derive(Debug)]
pub struct ReadDouble<'a> {
    endian: &'a Endian
}

impl<'a> ReadDouble<'a> {
    pub fn new(endian: &'a Endian) -> Self { 
        Self { endian } 
    }
}

impl<'a> DataOps for ReadDouble<'a> {
    type Out = f64;

    // fn read(&self, start: usize, bytes: &[u8]) -> Result<Self::Out, TryFromSliceError> {
    fn read(&self, start: usize, bytes: &[u8]) -> Option<Self::Out> {
        bytes[start..(start + 8)] 
            .try_into()
            .map(|double_bytes| {
                self.endian.convert_f64(double_bytes)
            })
            .ok() 
    }

    fn size(&self) -> usize {
        // self.data_size.size()
        8
    }
}
