use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Kind64 {
    Buffer,
    Fixed64,
    SFixed64,
    Double,
}

impl Kind64 {
    pub fn toggle(self) -> Self {
        match self {
            Kind64::Buffer => Kind64::Fixed64,
            Kind64::Fixed64 => Kind64::SFixed64,
            Kind64::SFixed64 => Kind64::Double,
            Kind64::Double => Kind64::Buffer,
        }
    }

    pub fn to_string(self) -> &'static str {
        match self {
            Kind64::Buffer => "raw",
            Kind64::Fixed64 => "unsign",
            Kind64::SFixed64 => "sign",
            Kind64::Double => "double",
        }
    }

    pub fn get_value_as_string(self, buffer: &[u8]) -> String {
        match self {
            Kind64::Buffer => crate::format::format_as_ascii_and_hex(buffer),
            Kind64::Fixed64 => {
                let mut cursor = Cursor::new(buffer);
                cursor.read_i64::<LittleEndian>().unwrap().to_string()
            }
            Kind64::SFixed64 => {
                let mut cursor = Cursor::new(buffer);
                cursor.read_u64::<LittleEndian>().unwrap().to_string()
            }
            Kind64::Double => {
                let mut cursor = Cursor::new(buffer);
                cursor.read_f64::<LittleEndian>().unwrap().to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggles_itself_in_cycle() {
        let kind = Kind64::Buffer;

        let actual = kind.toggle().toggle().toggle().toggle();

        assert_eq!(actual, kind);
    }
}
