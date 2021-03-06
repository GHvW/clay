use std::convert::TryInto;

use crate::shapes::{ Point, Polygon };
use crate::primitive_readers::{ ReadInt, ReadDouble, DataOps };
use crate::byte_reader::ByteReader;
use crate::shape_readers::point::PointR;
use crate::shape_readers::bounds_box::BoxR;
use crate::shapes::BoundingBox;
use crate::record_header::RecordHeaderR;


#[derive(Debug, PartialEq)]
pub struct PolygonStats {
    pub bounds_box: BoundingBox,
    pub parts_count: i32,
    pub points_count: i32 
}

impl PolygonStats {
    pub fn new(bounds_box: BoundingBox, parts_count: i32, points_count: i32) -> Self {
        Self {
            bounds_box,
            parts_count,
            points_count
        }
    }
}


pub struct PolygonStatsR<'a> {
    box_reader: BoxR<'a>,
    int_reader: &'a ReadInt
}

impl<'a> PolygonStatsR<'a> {
    pub fn new(box_reader: BoxR<'a>, int_reader: &'a ReadInt) -> Self {
        Self {
            box_reader,
            int_reader
        }
    }
}

impl<'a> DataOps for PolygonStatsR<'a> {
    type Out = PolygonStats;

    fn read(&self, start: usize, bytes: &[u8]) -> Option<Self::Out> {
        let box_size = self.box_reader.size();
        let b = self.box_reader.read(start, bytes)?;
        let f = self.int_reader.read(start + box_size, bytes)?;
        let s = self.int_reader.read(start + box_size + 4, bytes)?;

        Some(PolygonStats::new(b, f, s))
    }

    fn size(&self) -> usize {
        self.box_reader.size() + 8 // two Ints = 8 bytes
    }
}



pub struct PolygonPointsR<'a> {
    part_reader: ByteReader<'a, ReadInt>,
    point_reader: ByteReader<'a, PointR<'a>>
}

impl<'a> PolygonPointsR<'a> {
    pub fn new(parts_count: usize, points_count: usize, int_reader: &'a ReadInt, point_reader: &'a PointR) -> Self {
        Self {
            part_reader: ByteReader::new(int_reader, parts_count),
            point_reader: ByteReader::new(point_reader, points_count) // TODO better way to handle than unwrap?
        }
    }
}

impl<'a> DataOps for PolygonPointsR<'a> {
    type Out = (Vec<i32>, Vec<Point>);

    fn read(&self, start: usize, bytes: &[u8]) -> Option<Self::Out> {
        let parts = self.part_reader.read(start, bytes)?;
        let points = self.point_reader.read(start + self.part_reader.size(), bytes)?;

        Some((parts, points))
    }

    fn size(&self) -> usize {
        self.point_reader.size() + self.part_reader.size()
    }
}

pub struct PolygonRecordData {
    pub size: usize,
    pub polygon: Polygon
}

impl PolygonRecordData {
    pub fn new(size: usize, polygon: Polygon) -> PolygonRecordData {
        PolygonRecordData {
            size, polygon
        }
    }
}


pub struct PolygonRecordR<'a> {
    record_header_reader: RecordHeaderR<'a>,
    stats_reader: PolygonStatsR<'a>,
    int_reader: &'a ReadInt,
    point_reader: PointR<'a>
}

impl<'a> PolygonRecordR<'a> {
    pub fn new(record_header_reader: RecordHeaderR<'a>, stats_reader: PolygonStatsR<'a>, int_reader: &'a ReadInt, point_reader: PointR<'a>) -> Self {
        Self {
            record_header_reader,
            stats_reader,
            int_reader,
            point_reader
        }
    }

    pub fn read_record(&self, start: usize, bytes: &[u8]) -> Option<PolygonRecordData> {
        let header = self.record_header_reader.read(start, bytes)?;
        let header_size = self.record_header_reader.size();
        let _shape_type = self.int_reader.read(start + header_size, bytes)?;
        let stats = self.stats_reader.read(start + header_size + self.int_reader.size(), bytes)?;

        let points_reader = 
            PolygonPointsR::new(
                stats.parts_count as usize, 
                stats.points_count as usize, 
                &self.int_reader, 
                &self.point_reader);

        let (parts, points) = points_reader.read(start + header_size + self.int_reader.size() + self.stats_reader.size(), bytes)?;

        // let maybe_content_len = header_size + self.int_reader.size() + self.stats_reader.size() + points_reader.size();
        Some(
            PolygonRecordData::new(
                ((header.content_length * 2) + 8) as usize, // size is in 16 bit words, does not include header size
                // maybe_content_len as usize,
                Polygon::new(stats.bounds_box, stats.parts_count, stats.points_count, parts, points)))
    }
}

// impl<'a> DataOps for PolygonR<'a> {
//     type Out = (usize, Polygon);

//     fn read(&self, start: usize, bytes: &[u8]) -> Option<Self::Out> {
//         let stats = self.stats_reader.read(start, bytes)?;

//         let points_reader = 
//             PolygonPointsR::new(
//                 stats.parts_count as usize, 
//                 stats.points_count as usize, 
//                 &self.int_reader, 
//                 &self.point_reader);

//         let (parts, points) = points_reader.read(start + self.stats_reader.size(), bytes)?;

//         Some((points_reader.size(), Polygon::new(stats.bounds_box, stats.parts_count, stats.points_count, parts, points)))
//     }

//     fn size(&self) -> usize {
//         self.point_reader.size() + self.stats_reader.size()
//     }
// }


// pub struct PolygonRecordR<'a> {
//     int_reader: &'a ReadInt,
//     polygon_reader: PolygonR<'a>
// }

// impl<'a> PolygonRecordR<'a> {
//     pub fn new(int_reader: &'a ReadInt, polygon_reader: PolygonR<'a>) -> Self {
//         Self {
//             int_reader,
//             polygon_reader
//         }
//     }

//     // pub fn read_many(&self, count: usize) -> ByteReader<PolygonRecordR> {
//     //     ByteReader::new(self, count)
//     // }
// }

// impl<'a> DataOps for PolygonRecordR<'a> {
//     type Out = (i32, Polygon);

//     fn read(&self, start: usize, bytes: &[u8]) -> Option<Self::Out> {
//         let shape_type = self.int_reader.read(start, bytes)?;
//         // TODO check shape type?
//         let polygon = self.polygon_reader.read(start + self.int_reader.size(), bytes)?;
//         Some((shape_type, polygon))
//     }

//     fn size(&self) -> usize {
//         self.int_reader.size() + self.polygon_reader.size()
//     }
// }


#[cfg(test)]
mod tests {
    use super::*;

    use crate::endian::Endian;

    #[test]
    fn stats_reader_reads_stats() {
        let x_min = f64::to_le_bytes(55.55);
        let y_min = f64::to_le_bytes(88.88);
        let x_max = f64::to_le_bytes(100.9);
        let y_max = f64::to_le_bytes(120.2);
        let parts_count = i32::to_le_bytes(15);
        let points_count = i32::to_le_bytes(23);
        let bytes = [[x_min, y_min, x_max, y_max].concat(), 
                         [parts_count, 
                          points_count].concat()].concat();


        let int_reader = ReadInt::new(Endian::Little);
        let double_reader = ReadDouble::new(Endian::Little);

        let box_reader = BoxR::new(&double_reader);

        let stats_reader = PolygonStatsR::new(box_reader, &int_reader);

        let actual = stats_reader.read(0, &bytes).unwrap();

        let expected = 
            PolygonStats::new(
                BoundingBox::new(55.55, 88.88, 100.9, 120.2),
                15,
                23);

        assert_eq!(expected, actual);
    }


    #[test]
    fn points_reader_reads_points_and_parts() {

        let parts_bytes = [i32::to_le_bytes(333), i32::to_le_bytes(777)].concat();
        let points_bytes = [f64::to_le_bytes(-100.25), f64::to_le_bytes(-150.75),
                            f64::to_le_bytes(-75.88), f64::to_le_bytes(25.99),
                            f64::to_le_bytes(110.11), f64::to_le_bytes(175.33),
                            f64::to_le_bytes(144.44), f64::to_le_bytes(-55.55)].concat();

        let bytes = [parts_bytes, points_bytes].concat();


        let int_reader = ReadInt::new(Endian::Little);
        let double_reader = ReadDouble::new(Endian::Little);

        let point_reader = PointR::new(&double_reader);

        let stats_reader = PolygonPointsR::new(2, 4, &int_reader, &point_reader);

        let (actual_parts, actual_points) = stats_reader.read(0, &bytes).unwrap();

        let expected_parts = vec![333, 777];

        let expected_points = vec![Point::new(-100.25, -150.75), 
                                   Point::new(-75.88, 25.99), 
                                   Point::new(110.11, 175.33), 
                                   Point::new(144.44, -55.55)];

        assert_eq!(expected_parts, actual_parts);
        assert_eq!(expected_points, actual_points);
    }
}