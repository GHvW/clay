use crate::primitive_readers::{ ReadInt, ReadDouble, DataOps };
use crate::byte_reader::ByteReader;

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


pub struct MainFileHeaderR<'a> {
    // int_reader: &'a ReadInt
    init_reader: ByteReader<'a, ReadInt>,
    version_and_typer: ByteReader<'a, ReadInt>,
    bounds_reader: ByteReader<'a, ReadDouble>
}

impl<'a> MainFileHeaderR<'a> {
    pub fn new(little_int_reader: &'a ReadInt, big_int_reader: &'a ReadInt, double_reader: &'a ReadDouble) -> Self {
        Self {
            init_reader: ByteReader::new(big_int_reader, 8),
            version_and_typer: ByteReader::new(little_int_reader, 2),
            bounds_reader: ByteReader::new(double_reader, 8)
        }
    }
}

impl<'a> DataOps for MainFileHeaderR<'a> {
    type Out = MainFileHeader;

    fn read(&self, start: usize, bytes: &[u8]) -> Option<Self::Out> {
        let init = self.init_reader.read(start, bytes)?;
        let v_t = self.version_and_typer.read(start + self.init_reader.size(), bytes)?;
        let bounds = self.bounds_reader.read(start + self.init_reader.size() + self.version_and_typer.size(), bytes)?;

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
                bounds.get(4).map(|x| *x),
                bounds.get(5).map(|x| *x),
                bounds.get(6).map(|x| *x),
                bounds.get(7).map(|x| *x)
            )))
    }

    fn size(&self) -> usize {
        self.init_reader.size() + self.version_and_typer.size() + self.bounds_reader.size()
    }
}