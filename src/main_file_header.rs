
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

pub struct MainFileHeader {
    pub file_code: i32,
    pub file_length: i32,
    pub bounds: FileHeaderBounds
}
