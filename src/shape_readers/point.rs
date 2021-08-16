// use crate::shapes::Point;
// use crate::primitive_readers::{ ReadDouble, DataOps };
// use crate::byte_reader::ByteReader;

// pub struct PointR<'a> {
//     byte_reader: ByteReader<'a, ReadDouble>
// }

// impl<'a> PointR<'a> {
//     pub fn new(double_reader: &'a ReadDouble) -> Self {
//         Self { 
//             byte_reader: ByteReader::new(double_reader, 2)
//         }
//     }
// }

// impl<'a> DataOps for PointR<'a> {
//     type Out = Point;

//     fn read(&self, start: usize, bytes: &[u8]) -> Option<Self::Out> {
//         self.byte_reader
//             .read(start, bytes)
//             .map(|vec| {
//                 Point::new(vec[0], vec[1])
//             })
//     }

//     fn size(&self) -> usize {
//         self.byte_reader.size()
//     }
// }



// #[cfg(test)]
// mod tests {
//     use super::*;
//     // use crate::primitive_readers::ReadDouble;
//     use crate::endian::Endian;

//     #[test]
//     fn pointr_reads_point_from_zero() {
//         // Arrange
//         // x [64, 200, 28, 214, 230, 49, 248, 161]
//         // y [64, 234, 134, 63, 154, 107, 80, 177]
//         // let x_bytes = [0b01000000, 0b11001000, 0b00011100, 0b11010110, 0b11100110, 0b00110001, 0b11111000, 0b10100001];
//         // let y_bytes = [0b01000000, 0b11101010, 0b10000110, 0b00111111, 0b10011010, 0b01101011, 0b01010000, 0b10110001];
//         let bytes = [0b01000000, 0b11001000, 0b00011100, 0b11010110, 0b11100110, 0b00110001, 0b11111000, 0b10100001,
//                      0b01000000, 0b11101010, 0b10000110, 0b00111111, 0b10011010, 0b01101011, 0b01010000, 0b10110001];

//         let double_r = ReadDouble::new(Endian::Big);
//         let reader = PointR::new(&double_r);

//         // Act
//         let result = reader.read(0, &bytes).unwrap();

//         let expected = Point::new(12345.6789, 54321.9876);
//         // Assert
//         assert_eq!(expected, result);
//     }

//     #[test]
//     fn pointr_reads_point_from_middle_of_array() {
//         // Arrange
//         // x [64, 200, 28, 214, 230, 49, 248, 161]
//         // y [64, 234, 134, 63, 154, 107, 80, 177]
//         // let x_bytes = [0b01000000, 0b11001000, 0b00011100, 0b11010110, 0b11100110, 0b00110001, 0b11111000, 0b10100001];
//         // let y_bytes = [0b01000000, 0b11101010, 0b10000110, 0b00111111, 0b10011010, 0b01101011, 0b01010000, 0b10110001];
//         let bytes = [0b11111111, 
//                      0b01000000, 0b11001000, 0b00011100, 0b11010110, 0b11100110, 0b00110001, 0b11111000, 0b10100001,
//                      0b01000000, 0b11101010, 0b10000110, 0b00111111, 0b10011010, 0b01101011, 0b01010000, 0b10110001,
//                      0b11111111];

//         let double_r = ReadDouble::new(Endian::Big);
//         let reader = PointR::new(&double_r);

//         // Act
//         let result = reader.read(1, &bytes).unwrap();

//         let expected = Point::new(12345.6789, 54321.9876);
//         // Assert
//         assert_eq!(expected, result);
//     }
// }