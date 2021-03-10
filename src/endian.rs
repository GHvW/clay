
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Endian {
    Big,
    Little
}

impl Endian {

    // TODO - make private?
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
}



#[cfg(test)]
mod tests {
    // use super::*;

}

