use std::convert::TryInto;
use std::array::TryFromSliceError;

pub enum Endian {
    Big,
    Little
}

impl Endian {

    pub fn read_int32(&self, buffer: [u8; 4]) -> i32 {
        match self {
            Endian::Big => i32::from_be_bytes(buffer),
            Endian::Little => i32::from_le_bytes(buffer)
        }
    }

    pub fn read_f64(&self, buffer: [u8; 8]) -> f64 {
        match self {
            Endian::Big => f64::from_be_bytes(buffer),
            Endian::Little => f64::from_le_bytes(buffer)
        }
    }
}

pub fn convert_int(endian: Endian, bytes: &[u8]) -> Result<(i32, &[u8]), TryFromSliceError> {
    bytes
        .try_into()
        .map(|int_bytes| {
            let i = endian.read_int32(int_bytes);
            (i, &bytes[0..4])
        })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_int_test() {
        let initial = [0b00100001, 0b00100001, 0b11110111, 0b10111110];

        let result = convert_int(Endian::Big, &initial).unwrap().0;

        let expected =  555_874_238;
        assert_eq!(result, expected);
    }
}

