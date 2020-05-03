use crate::{error::EmptyError, key::Key, varint, wire_type::WireType};

#[derive(Debug, Clone)]
pub enum Data {
    Varint {
        field_number: u128,
        value: u128,
    },
    Fixed64 {
        field_number: u128,
        buffer: Vec<u8>,
    },
    Fixed32 {
        field_number: u128,
        buffer: Vec<u8>,
    },
    Chunk {
        field_number: u128,
        buffer: Vec<u8>,
    },
    Message {
        field_number: u128,
        items: Vec<Data>,
    },
}

pub fn take_data(key: Key, buffer: &[u8]) -> Result<(Data, &[u8]), EmptyError> {
    use Data::*;
    let field_number = key.field_number;
    match key.wire_type {
        WireType::Varint => {
            let (varint, rest) = varint::take_varint(buffer);
            Ok((
                Varint {
                    field_number,
                    value: varint,
                },
                rest,
            ))
        }

        WireType::Size64 => {
            if buffer.len() < 8 {
                return Err(EmptyError {});
            }

            let (buffer, rest) = buffer.split_at(8);
            Ok((
                Fixed64 {
                    field_number,
                    buffer: buffer.into(),
                },
                rest,
            ))
        }
        WireType::LengthDelimited => {
            let (buffer_size, rest) = varint::take_varint(buffer);
            Ok((
                Chunk {
                    field_number,
                    buffer: rest[0..(buffer_size as usize)].into(),
                },
                &rest[buffer_size as usize..],
            ))
        }

        WireType::StartGroup => Err(EmptyError {}),

        WireType::EndGroup => Err(EmptyError {}),

        WireType::Size32 => {
            if buffer.len() < 4 {
                return Err(EmptyError {});
            }

            let (buffer, rest) = buffer.split_at(4);
            Ok((
                Fixed32 {
                    field_number,
                    buffer: buffer.into(),
                },
                rest,
            ))
        }

        WireType::Invalid => Err(EmptyError {}),
    }
}
