use crate::primitive_readers::{ ReadInt, ReadDouble, DataOps };
use crate::byte_reader::ByteReader;

#[derive(Debug, PartialEq)]
pub struct FileHeaderBounds {
    pub x_min: f64,
    pub y_min: f64,
    pub x_max: f64,
    pub y_max: f64,
    pub z_min: Option<f64>,
    pub z_max: Option<f64>,
    pub m_min: Option<f64>,
    pub m_max: Option<f64>
}

impl FileHeaderBounds {
    pub fn new(x_min: f64, y_min: f64, x_max: f64, y_max: f64, z_min: Option<f64>, z_max: Option<f64>, m_min: Option<f64>, m_max: Option<f64>) -> Self {
        FileHeaderBounds {
            x_min,
            y_min,
            x_max,
            y_max,
            z_min,
            z_max,
            m_min,
            m_max
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct MainFileHeader {
    pub file_code: i32,
    pub file_length: i32,
    pub version: i32,
    pub shape_type: i32,
    pub bounds: FileHeaderBounds
}

impl MainFileHeader {
    pub fn new(file_code: i32, file_length: i32, version: i32, shape_type: i32, bounds: FileHeaderBounds) -> Self {
        MainFileHeader { 
            file_code, 
            file_length, 
            version,
            shape_type,
            bounds 
        }
    }
}


pub struct MainFileHeaderR {
    init_reader: ByteReader<ReadInt>,
    version_and_typer: ByteReader<ReadInt>,
    bounds_reader: ByteReader<ReadDouble>
}

impl MainFileHeaderR {
    pub fn new(little_int_reader: ReadInt, big_int_reader: ReadInt, double_reader: ReadDouble) -> Self {
        Self {
            init_reader: ByteReader::new(big_int_reader, 7),
            version_and_typer: ByteReader::new(little_int_reader, 2),
            bounds_reader: ByteReader::new(double_reader, 8)
        }
    }
}

impl DataOps for MainFileHeaderR {
    type Out = MainFileHeader;

    fn read(&self, start: usize, bytes: &[u8]) -> Option<Self::Out> {
        let init = self.init_reader.read(start, bytes)?;
        let v_t = self.version_and_typer.read(start + self.init_reader.size(), bytes)?;
        let bounds = self.bounds_reader.read(start + self.init_reader.size() + self.version_and_typer.size(), bytes)?;

        let bounds4 = bounds[4];
        let bounds5 = bounds[5];
        let bounds6 = bounds[6];
        let bounds7 = bounds[7];

        Some(MainFileHeader::new(
            init[0], 
            init[6],
            v_t[0],
            v_t[1],
            FileHeaderBounds::new(
                bounds[0],
                bounds[1],
                bounds[2],
                bounds[3],
                if bounds4 == 0.0 { None } else { Some(bounds4) },
                if bounds5 == 0.0 { None } else { Some(bounds5) },
                if bounds6 == 0.0 { None } else { Some(bounds6) },
                if bounds7 == 0.0 { None } else { Some(bounds7) },
            )))
    }

    fn size(&self) -> usize {
        self.init_reader.size() + self.version_and_typer.size() + self.bounds_reader.size()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::endian::Endian;

    #[test]
    fn main_file_header_r_works() {
        // Arrange
        let main = [
            i32::to_be_bytes(9994), 
            i32::to_be_bytes(0), 
            i32::to_be_bytes(0), 
            i32::to_be_bytes(0), 
            i32::to_be_bytes(0), 
            i32::to_be_bytes(0),
            i32::to_be_bytes(200),
            i32::to_le_bytes(1000),
            i32::to_le_bytes(5)
        ].concat();

        let bounds = [
            f64::to_le_bytes(5.5),
            f64::to_le_bytes(10.5),
            f64::to_le_bytes(20.5),
            f64::to_le_bytes(30.5),
            f64::to_le_bytes(0.0),
            f64::to_le_bytes(0.0),
            f64::to_le_bytes(0.0),
            f64::to_le_bytes(0.0),
        ].concat();

        let bytes = [main, bounds].concat();

        let little = ReadInt::new(Endian::Little);
        let big = ReadInt::new(Endian::Big);
        let double = ReadDouble::new(Endian::Little);

        let reader = 
            MainFileHeaderR::new(
                little,
                big,
                double);

        // Act
        let actual = reader.read(0, &bytes).unwrap();

        // Assert
        let expected = 
            MainFileHeader::new(
                9994, 
                200, 
                1000, 
                5, 
                FileHeaderBounds::new(5.5, 10.5, 20.5, 30.5, None, None, None, None));

        assert_eq!(expected, actual);
    }
}