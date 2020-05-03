#[derive(Copy, Clone, Debug, PartialEq)]
pub enum KindVarint {
    ZigZag,
    Unsigned,
}

impl KindVarint {
    pub fn toggle(self) -> Self {
        match self {
            KindVarint::Unsigned => KindVarint::ZigZag,
            KindVarint::ZigZag => KindVarint::Unsigned,
        }
    }

    pub fn to_string(self) -> &'static str {
        match self {
            KindVarint::ZigZag => "zigzag",
            KindVarint::Unsigned => "unsign",
        }
    }

    pub fn get_value_as_string(self, varint: u128) -> String {
        match self {
            KindVarint::ZigZag => {
                let varint = varint as i128;
                // TODO: It looks like the logic is similar, need to join it
                ((varint >> 1) ^ -(varint & 0x1)).to_string()
            }
            KindVarint::Unsigned => varint.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_works_in_cycle() {
        let kind = KindVarint::ZigZag;

        let actual = kind.toggle().toggle();

        assert_eq!(actual, kind);
    }

    #[test]
    fn decodes_zigzag_correctly_64() {
        assert_eq!("0", KindVarint::ZigZag.get_value_as_string(0));
        assert_eq!("-1", KindVarint::ZigZag.get_value_as_string(1));
        assert_eq!("1", KindVarint::ZigZag.get_value_as_string(2));
        assert_eq!("-2", KindVarint::ZigZag.get_value_as_string(3));
        assert_eq!(
            "2147483647",
            KindVarint::ZigZag.get_value_as_string(4294967294)
        );
        assert_eq!(
            "-2147483648",
            KindVarint::ZigZag.get_value_as_string(4294967295)
        );
    }
}
