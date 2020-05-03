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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_correct_from_implementation() {
        let wire_type: WireType = From::<u128>::from(0);
        assert_eq!(wire_type, WireType::Varint);

        let wire_type: WireType = From::<u128>::from(1);
        assert_eq!(wire_type, WireType::Size64);

        let wire_type: WireType = From::<u128>::from(2);
        assert_eq!(wire_type, WireType::LengthDelimited);

        let wire_type: WireType = From::<u128>::from(3);
        assert_eq!(wire_type, WireType::StartGroup);

        let wire_type: WireType = From::<u128>::from(4);
        assert_eq!(wire_type, WireType::EndGroup);

        let wire_type: WireType = From::<u128>::from(5);
        assert_eq!(wire_type, WireType::Size32);
    }
}
