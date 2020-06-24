use crate::wire_type::WireType;

pub struct Key {
    pub wire_type: WireType,
    pub field_number: u128,
}

pub fn take_key(buffer: &[u8]) -> (Key, &[u8]) {
    let (varint, rest) = crate::varint::take_varint(buffer);
    (
        Key {
            wire_type: (varint & 0x07).into(),
            field_number: varint >> 3,
        },
        rest,
    )
}
