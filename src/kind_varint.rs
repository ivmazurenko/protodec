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
