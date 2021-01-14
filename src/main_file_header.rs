
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
    pub bounds: FileHeaderBounds
}

impl MainFileHeader {
    pub fn new(file_code: i32, file_length: i32, bounds: FileHeaderBounds) -> Self {
        MainFileHeader { 
            file_code, 
            file_length, 
            bounds 
        }
    }
}