use std::convert::TryInto;
use std::array::TryFromSliceError;

pub enum Endian {
    Big,
    Little
}

impl Endian {

    pub fn convert_int32(&self, buffer: [u8; 4]) -> i32 {
        match self {
            Endian::Big => i32::from_be_bytes(buffer),
            Endian::Little => i32::from_le_bytes(buffer)
        }
    }

    pub fn convert_f64(&self, buffer: [u8; 8]) -> f64 {
        match self {
            Endian::Big => f64::from_be_bytes(buffer),
            Endian::Little => f64::from_le_bytes(buffer)
        }
    }

    // TODO - refactor these readers later to use one algo?
    pub fn read_int<'a>(&self, bytes: &'a[u8]) -> Result<(i32, &'a[u8]), TryFromSliceError> {
        bytes[0..4] // & here automagically derefed by .try_into?
            .try_into()
            .map(|int_bytes| {
                let int = self.convert_int32(int_bytes);
                (int, &bytes[4..])
            })
    }

    pub fn read_double<'a>(&self, bytes: &'a[u8]) -> Result<(f64, &'a[u8]), TryFromSliceError> {
        bytes[0..8]
            .try_into()
            .map(|double_bytes| {
                let double = self.convert_f64(double_bytes);
                (double, &bytes[8..])
            })
    }
}

// pub fn convert_int(endian: Endian, bytes: &[u8]) -> Result<(i32, &[u8]), TryFromSliceError> {
//     bytes[0..4] // & here automagically derefed by .try_into?
//         .try_into()
//         .map(|int_bytes| {
//             let i = endian.read_int32(int_bytes);
//             (i, &bytes[4..])
//         })
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_big_endian_int_perfect_size_test() {
        let initial = [0b00100001, 0b00100001, 0b11110111, 0b10111110];

        // let result = convert_int(Endian::Big, &initial).unwrap().0;
        let result = Endian::Big.read_int(&initial).unwrap().0;

        let expected =  555_874_238;
        assert_eq!(result, expected);
    }

    #[test]
    fn convert_big_endian_int_with_extra_bytes_test() {
        let initial = [0b00100001, 0b00100001, 0b11110111, 0b10111110, 0b00110001, 0b00100011, 0b11110011, 0b10011110];

        // let (result, _) = convert_int(Endian::Big, &initial).unwrap();
        let (result, _) = Endian::Big.read_int(&initial).unwrap();

        let expected =  555_874_238;
        assert_eq!(result, expected);
    }

    #[test]
    fn convert_big_endian_int_with_extra_bytes_has_correct_remaining_bytes_test() { //TODO - make bytes different at end
        let initial = [0b00100001, 0b00100001, 0b11110111, 0b10101110, 0b00110001, 0b00100011, 0b11110011, 0b10011110];

        // let (_, rest) = convert_int(Endian::Big, &initial).unwrap();
        let (_, rest) = Endian::Big.read_int(&initial).unwrap();

        let expected = [0b00110001, 0b00100011, 0b11110011, 0b10011110];

        assert_eq!(rest, expected);
    }


    #[test]
    fn convert_little_endian_int_perfect_size_test() {
        let initial = [0b10111110, 0b11110111, 0b00100001, 0b00100001];

        let result = Endian::Little.read_int(&initial).unwrap().0;

        let expected =  555_874_238;
        assert_eq!(result, expected);
    }


    // ************************* double tests **********************
    #[test]
    fn convert_big_endian_double_perfect_size() {
        // 12345.6789
        // [64, 200, 28, 214, 230, 49, 248, 161]
        let initial = [0b01000000, 0b11001000, 0b00011100, 0b11010110, 0b11100110, 0b00110001, 0b11111000, 0b10100001];

        let result = Endian::Big.read_double(&initial).unwrap().0;

        let expected =  12345.6789;
        assert_eq!(result, expected);
    }


    #[test]
    fn convert_big_endian_double_extra_bytes() {
        // 12345.6789
        // [64, 200, 28, 214, 230, 49, 248, 161]
        let initial = [0b01000000, 0b11001000, 0b00011100, 0b11010110, 0b11100110, 0b00110001, 0b11111000, 0b10100001,
        0b01001100, 0b11011100, 0b01011100, 0b11000100, 0b10100110, 0b00110101, 0b11011000, 0b10101001];

        let result = Endian::Big.read_double(&initial).unwrap().0;

        let expected =  12345.6789;
        assert_eq!(result, expected);
    }


    #[test]
    fn convert_big_endian_double_extra_bytes_has_correct_remaining_bytes() {
        // 12345.6789
        // [64, 200, 28, 214, 230, 49, 248, 161]
        let initial = [0b01000000, 0b11001000, 0b00011100, 0b11010110, 0b11100110, 0b00110001, 0b11111000, 0b10100001,
        0b01001100, 0b11011100, 0b01011100, 0b11000100, 0b10100110, 0b00110101, 0b11011000, 0b10101001];

        let (_, rest) = Endian::Big.read_double(&initial).unwrap();

        let expected = [0b01001100, 0b11011100, 0b01011100, 0b11000100, 0b10100110, 0b00110101, 0b11011000, 0b10101001];
        assert_eq!(rest, expected);
    }


    #[test]
    fn convert_little_endian_double_perfect_size() {
        let initial = [0b10100001, 0b11111000, 0b00110001, 0b11100110, 0b11010110, 0b00011100, 0b11001000, 0b01000000];

        let result = Endian::Little.read_double(&initial).unwrap().0;

        let expected =  12345.6789;
        assert_eq!(result, expected);
    }
}

