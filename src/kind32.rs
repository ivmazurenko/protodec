use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Kind32 {
    Buffer,
    Fixed32,
    SFixed32,
    Float,
}

impl Kind32 {
    pub fn toggle(self) -> Self {
        match self {
            Kind32::Buffer => Kind32::Fixed32,
            Kind32::Fixed32 => Kind32::SFixed32,
            Kind32::SFixed32 => Kind32::Float,
            Kind32::Float => Kind32::Buffer,
        }
    }

    pub fn to_string(self) -> &'static str {
        match self {
            Kind32::Buffer => "raw",
            Kind32::Fixed32 => "unsign",
            Kind32::SFixed32 => "sign",
            Kind32::Float => "float",
        }
    }
    pub fn get_value_as_string(self, buffer: &[u8]) -> String {
        match self {
            Kind32::Buffer => crate::format::format_as_ascii_and_hex(buffer),
            Kind32::Fixed32 => {
                let mut cursor = Cursor::new(buffer);
                cursor.read_i32::<LittleEndian>().unwrap().to_string()
            }
            Kind32::SFixed32 => {
                let mut cursor = Cursor::new(buffer);
                cursor.read_u32::<LittleEndian>().unwrap().to_string()
            }
            Kind32::Float => {
                let mut cursor = Cursor::new(buffer);
                cursor.read_f32::<LittleEndian>().unwrap().to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggles_itself_in_cycle() {
        let kind = Kind32::Buffer;

        let actual = kind.toggle().toggle().toggle().toggle();

        assert_eq!(actual, kind);
    }
}
