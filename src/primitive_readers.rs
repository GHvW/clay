use std::convert::TryInto;
use crate::endian::Endian;


pub trait DataOps {
    type Out;

    fn read(&self, start: usize, bytes: &[u8]) -> Option<Self::Out>;
    fn size(&self) -> usize;
}


#[derive(Debug)]
pub struct ReadInt {
    endian: Endian
}

impl ReadInt {
    pub fn new(endian: Endian) -> Self {
        Self { endian }
    }
}

impl DataOps for ReadInt {
    type Out = i32;

    fn read(&self, start: usize, bytes: &[u8]) -> Option<Self::Out> {
        bytes.get(start..(start + 4))
            ?.try_into()
            .map(|int_bytes| {
                self.endian.convert_int32(int_bytes)
            })
            .ok()
    }

    fn size(&self) -> usize {
        4
    }
}


#[derive(Debug)]
pub struct ReadDouble {
    endian: Endian
}

impl ReadDouble {
    pub fn new(endian: Endian) -> Self { 
        Self { endian } 
    }
}

impl DataOps for ReadDouble {
    type Out = f64;

    fn read(&self, start: usize, bytes: &[u8]) -> Option<Self::Out> {
        bytes.get(start..(start + 8)) 
            ?.try_into()
            .map(|double_bytes| {
                self.endian.convert_f64(double_bytes)
            })
            .ok() 
    }

    fn size(&self) -> usize {
        8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_big_endian_int_perfect_size_test() {
        let initial = [0b00100001, 0b00100001, 0b11110111, 0b10111110];

        let result = ReadInt::new(Endian::Big).read(0, &initial).unwrap();

        let expected =  555_874_238;
        assert_eq!(result, expected);
    }


    #[test]
    fn convert_big_endian_int_with_extra_bytes_test() {
        let initial = [0b00100001, 0b00100001, 0b11110111, 0b10111110, 0b00110001, 0b00100011, 0b11110011, 0b10011110];

        // let (result, _) = convert_int(Endian::Big, &initial).unwrap();
        let result = ReadInt::new(Endian::Big).read(0, &initial).unwrap();

        let expected =  555_874_238;
        assert_eq!(result, expected);
    }


    #[test]
    fn convert_little_endian_int_perfect_size_test() {
        let initial = [0b10111110, 0b11110111, 0b00100001, 0b00100001];

        let result = ReadInt::new(Endian::Little).read(0, &initial).unwrap();

        let expected =  555_874_238;
        assert_eq!(result, expected);
    }


    // ************************* double tests **********************
    #[test]
    fn convert_big_endian_double_perfect_size() {
        // 12345.6789
        // [64, 200, 28, 214, 230, 49, 248, 161]
        let initial = [0b01000000, 0b11001000, 0b00011100, 0b11010110, 0b11100110, 0b00110001, 0b11111000, 0b10100001];

        let result = ReadDouble::new(Endian::Big).read(0, &initial).unwrap();

        let expected =  12345.6789;
        assert_eq!(result, expected);
    }


    #[test]
    fn convert_big_endian_double_extra_bytes() {
        // 12345.6789
        // [64, 200, 28, 214, 230, 49, 248, 161]
        let initial = [0b01000000, 0b11001000, 0b00011100, 0b11010110, 0b11100110, 0b00110001, 0b11111000, 0b10100001,
        0b01001100, 0b11011100, 0b01011100, 0b11000100, 0b10100110, 0b00110101, 0b11011000, 0b10101001];

        let result = ReadDouble::new(Endian::Big).read(0, &initial).unwrap();

        let expected =  12345.6789;
        assert_eq!(result, expected);
    }


    #[test]
    fn convert_little_endian_double_perfect_size() {
        let initial = [0b10100001, 0b11111000, 0b00110001, 0b11100110, 0b11010110, 0b00011100, 0b11001000, 0b01000000];

        let result = ReadDouble::new(Endian::Little).read(0, &initial).unwrap();

        let expected =  12345.6789;
        assert_eq!(result, expected);
    }
}