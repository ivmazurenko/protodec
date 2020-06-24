#[derive(Debug, PartialEq)]
pub enum WireType {
    Varint = 0,
    Size64 = 1,
    LengthDelimited = 2,
    StartGroup = 3,
    EndGroup = 4,
    Size32 = 5,
    Invalid,
}

impl From<u128> for WireType {
    fn from(val: u128) -> Self {
        match val {
            0 => WireType::Varint,
            1 => WireType::Size64,
            2 => WireType::LengthDelimited,
            3 => WireType::StartGroup,
            4 => WireType::EndGroup,
            5 => WireType::Size32,
            _ => WireType::Invalid,
        }
    }
}
