use crate::{data::Data, decode, kind32::Kind32, kind64::Kind64, kind_varint::KindVarint};
use uuid::Uuid;

pub enum ProtoDec {
    InitialState(InitialState),
    Decoding(DecodingState),
}

pub struct InitialState {
    pub input: String,
}

impl ProtoDec {
    pub fn new() -> Self {
        ProtoDec::InitialState(InitialState {
            input: String::new(),
        })
    }
}

#[derive(Debug, Clone)]
pub enum DecodingState {
    Varint {
        field_number: u128,
        value: u128,
        uuid: Uuid,
        kind: KindVarint,
    },

    Fixed64 {
        field_number: u128,
        buffer: Vec<u8>,
        uuid: Uuid,
        kind: Kind64,
    },

    Fixed32 {
        field_number: u128,
        buffer: Vec<u8>,
        uuid: Uuid,
        kind: Kind32,
    },

    Chunk {
        field_number: u128,
        buffer: Vec<u8>,
        uuid: Uuid,
    },

    Message {
        field_number: u128,
        items: Vec<DecodingState>,
        uuid: Uuid,
    },

    Utf8String {
        field_number: u128,
        value: String,
        uuid: Uuid,
    },
}

impl From<Data> for DecodingState {
    fn from(data: Data) -> Self {
        match data {
            Data::Varint {
                field_number,
                value,
            } => DecodingState::Varint {
                field_number,
                value,
                uuid: Uuid::new_v4(),
                kind: KindVarint::Unsigned,
            },

            Data::Fixed64 {
                field_number,
                buffer,
            } => DecodingState::Fixed64 {
                field_number,
                buffer,
                uuid: Uuid::new_v4(),
                kind: Kind64::Fixed64,
            },

            Data::Fixed32 {
                field_number,
                buffer,
            } => DecodingState::Fixed32 {
                field_number,
                buffer,
                uuid: Uuid::new_v4(),
                kind: Kind32::Fixed32,
            },

            Data::Chunk {
                field_number,
                buffer,
            } => DecodingState::Chunk {
                field_number,
                buffer,
                uuid: Uuid::new_v4(),
            },

            Data::Message {
                field_number,
                items,
            } => DecodingState::Message {
                field_number,
                items: items.iter().map(|d| Self::from(d.clone())).collect(),
                uuid: Uuid::new_v4(),
            },
        }
    }
}

impl DecodingState {
    pub fn decode_as_message(&mut self, ui_event_uuid: Uuid) {
        match self {
            DecodingState::Chunk {
                field_number,
                buffer,
                uuid,
                ..
            } => {
                if ui_event_uuid == *uuid {
                    if let Ok(data_items) = decode::decode_message(buffer) {
                        *self = Self::Message {
                            field_number: *field_number,
                            items: data_items.iter().map(|d| Self::from(d.clone())).collect(),
                            uuid: *uuid,
                        }
                    }
                }
            }
            DecodingState::Message { items, .. } => {
                for item in items {
                    item.decode_as_message(ui_event_uuid);
                }
            }
            _ => {}
        }
    }

    pub fn decode_as_utf8_string(&mut self, ui_event_uuid: Uuid) {
        match self {
            DecodingState::Chunk {
                field_number,
                buffer,
                uuid,
                ..
            } => {
                if ui_event_uuid == *uuid {
                    let decoded_string = String::from_utf8(buffer.to_vec());

                    if let Ok(decoded_string) = decoded_string {
                        *self = Self::Utf8String {
                            field_number: *field_number,
                            value: decoded_string,
                            uuid: *uuid,
                        }
                    }
                }
            }
            DecodingState::Message { items, .. } => {
                for item in items {
                    item.decode_as_utf8_string(ui_event_uuid);
                }
            }
            _ => {}
        }
    }

    pub fn toggle(&mut self, ui_event_uuid: Uuid) {
        match self {
            DecodingState::Varint { uuid, kind, .. } => {
                if ui_event_uuid == *uuid {
                    *kind = kind.toggle()
                }
            }

            DecodingState::Fixed32 { uuid, kind, .. } => {
                if ui_event_uuid == *uuid {
                    *kind = kind.toggle()
                }
            }

            DecodingState::Fixed64 { uuid, kind, .. } => {
                if ui_event_uuid == *uuid {
                    *kind = kind.toggle()
                }
            }

            DecodingState::Message { items, .. } => {
                for item in items {
                    item.toggle(ui_event_uuid);
                }
            }
            _ => {}
        }
    }

    pub fn get_formatted_field_number(&self) -> String {
        let field_number = match self {
            DecodingState::Varint { field_number, .. }
            | DecodingState::Fixed64 { field_number, .. }
            | DecodingState::Fixed32 { field_number, .. }
            | DecodingState::Chunk { field_number, .. }
            | DecodingState::Message { field_number, .. }
            | DecodingState::Utf8String { field_number, .. } => field_number,
        };

        format!("{:<2?}", field_number)
    }

    pub fn get_formatted_kind(&self) -> String {
        match self {
            DecodingState::Varint { kind, .. } => format!("<varint {}>", kind.to_string()),
            DecodingState::Fixed64 { kind, .. } => format!("<fix 64 {}>", kind.to_string()),
            DecodingState::Fixed32 { kind, .. } => format!("<fix 32 {}>", kind.to_string()),
            DecodingState::Chunk { .. } => "<chunk>".to_string(),
            DecodingState::Message { .. } => "<msg>".to_string(),
            DecodingState::Utf8String { .. } => "<utf-8>".to_string(),
        }
    }
}
