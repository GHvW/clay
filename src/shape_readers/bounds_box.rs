// use crate::byte_reader::{ ByteReader };
// use crate::primitive_readers::{ ReadDouble, DataOps };
// // use crate::shapes::BoundingBox;


// pub struct BoxR<'a> {
//     byte_reader: ByteReader<'a, ReadDouble>
// }

// impl<'a> BoxR<'a> {
//     pub fn new(ops: &'a ReadDouble) -> Self {
//         Self {
//             byte_reader: ByteReader::new(ops, 4)
//         }
//     }
// }

// // impl<'a> DataOps for BoxR<'a> {
// //     type Out = BoundingBox;

// //     fn read(&self, start: usize, bytes: &[u8]) -> Option<Self::Out> {
// //         self.byte_reader.read(start, bytes).map(|bounds| {
// //             BoundingBox::new(bounds[0], bounds[1], bounds[2], bounds[3])
// //         })
// //     }

// //     fn size(&self) -> usize {
// //         self.byte_reader.size()
// //     }
// // }


// #[cfg(test)]
// mod tests {
//     // use super::*;
//     // // use crate::primitive_readers::ReadDouble;
//     // use crate::endian::Endian;

//     // #[test]
//     // fn box_reader_reads_4_doubles() {
//     //     // [64, 200, 28, 214, 230, 49, 248, 161] - 12345.6789
//     //     // [64, 234, 134, 63, 154, 107, 80, 177] - 54321.9876
//     //     // [64, 64, 185, 153, 153, 153, 153, 154] - 33.45
//     //     // [64, 83, 191, 92, 40, 245, 194, 143] - 78.99
//     //     let bytes = [0b01000000, 0b11001000, 0b00011100, 0b11010110, 0b11100110, 0b00110001, 0b11111000, 0b10100001,
//     //                  0b01000000, 0b11101010, 0b10000110, 0b00111111, 0b10011010, 0b01101011, 0b01010000, 0b10110001,
//     //                  0b01000000, 0b01000000, 0b10111001, 0b10011001, 0b10011001, 0b10011001, 0b10011001, 0b10011010,
//     //                  0b01000000, 0b01010011, 0b10111111, 0b01011100, 0b00101000, 0b11110101, 0b11000010, 0b10001111];

//     //     let double_reader = ReadDouble::new(Endian::Big);
//     //     let box_reader = BoxR::new(&double_reader);

//     //     // let expected = vec![12345.6789, 54321.9876, 33.45, 78.99];
//     //     let expected = BoundingBox::new(12345.6789, 54321.9876, 33.45, 78.99);

//     //     let actual = box_reader.read(0, &bytes).unwrap();

//     //     assert_eq!(expected, actual);
//     // }

//     // #[test]
//     // fn box_reader_reads_4_doubles_from_middle_of_buffer() {
//     //     // [64, 200, 28, 214, 230, 49, 248, 161] - 12345.6789
//     //     // [64, 234, 134, 63, 154, 107, 80, 177] - 54321.9876
//     //     // [64, 64, 185, 153, 153, 153, 153, 154] - 33.45
//     //     // [64, 83, 191, 92, 40, 245, 194, 143] - 78.99
//     //     let bytes = [0b11111111,
//     //                  0b01000000, 0b11001000, 0b00011100, 0b11010110, 0b11100110, 0b00110001, 0b11111000, 0b10100001,
//     //                  0b01000000, 0b11101010, 0b10000110, 0b00111111, 0b10011010, 0b01101011, 0b01010000, 0b10110001,
//     //                  0b01000000, 0b01000000, 0b10111001, 0b10011001, 0b10011001, 0b10011001, 0b10011001, 0b10011010,
//     //                  0b01000000, 0b01010011, 0b10111111, 0b01011100, 0b00101000, 0b11110101, 0b11000010, 0b10001111,
//     //                  0b11111111];

//     //     let double_reader = ReadDouble::new(Endian::Big);
//     //     let box_reader = BoxR::new(&double_reader);

//     //     // let expected = vec![12345.6789, 54321.9876, 33.45, 78.99];
//     //     let expected = BoundingBox::new(12345.6789, 54321.9876, 33.45, 78.99);

//     //     let actual = box_reader.read(1, &bytes).unwrap();

//     //     assert_eq!(expected, actual);
//     // }
// }