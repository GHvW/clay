use std::convert::TryInto;
use std::array::TryFromSliceError;

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



pub trait MetadataOps<A> {
    fn read(&self, start: usize, bytes: &[u8]) -> Result<A, TryFromSliceError>;
    fn size(&self) -> usize;
}

#[derive(Debug)]
pub struct PrimitiveMetadata<A> {
    // pub data_size: DataSize,
    pub endian: Endian,
    phantom: std::marker::PhantomData<A>
}

impl<A> PrimitiveMetadata<A> {
    // pub fn new(data_size: DataSize, endian: Endian) -> Self {
    pub fn new(endian: Endian, phantom: std::marker::PhantomData<A>) -> Self {
        Self {
            // data_size,
            endian,
            phantom
        }
    }
}

impl MetadataOps<i32> for PrimitiveMetadata<i32> {

    fn read(&self, start: usize, bytes: &[u8]) -> Result<i32, TryFromSliceError> {
        println!("reading int");
        bytes[start..(start + 4)] 
            .try_into()
            .map(|int_bytes| {
                self.endian.convert_int32(int_bytes)
            })       
    }

    fn size(&self) -> usize {
        4
    }
}

impl MetadataOps<f64> for PrimitiveMetadata<f64> {

    fn read(&self, start: usize, bytes: &[u8]) -> Result<f64, TryFromSliceError> {
        println!("reading double");
        bytes[start..(start + 8)] 
            .try_into()
            .map(|double_bytes| {
                self.endian.convert_f64(double_bytes)
            })       
    }

    fn size(&self) -> usize {
        8
    }
}

