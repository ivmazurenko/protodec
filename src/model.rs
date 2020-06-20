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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data::*;
    use crate::*;
    //use iced::button;
    use protobuf::Message;

    #[test]
    fn converts_itself_into_message() {
        let mut string_wrapper = StringWrapper::new();
        string_wrapper.set_value("117".into());
        let buffer = string_wrapper.write_to_bytes().unwrap();

        let uuid = Uuid::new_v4();

        let mut state = DecodingState::Chunk {
            buffer,
            uuid,
            field_number: 123,
        };

        state.decode_as_message(uuid);
        assert_variant!(state, DecodingState::Message{..});
    }

    #[test]
    fn converts_internal_chunk_to_message() {
        let mut string_wrapper = StringWrapper::new();
        string_wrapper.set_value("117".into());
        let mut string_wrapper_wrapper = StringWrapperWrapper::new();
        string_wrapper_wrapper.set_value(string_wrapper);

        let buffer = string_wrapper_wrapper.write_to_bytes().unwrap();

        let uuid = Uuid::new_v4();

        let mut state = DecodingState::Chunk {
            buffer,
            uuid,
            field_number: 123,
        };

        state.decode_as_message(uuid);
        let mut clone = state.clone();

        let internal_uuid = if let DecodingState::Message { items, .. } = state {
            if let DecodingState::Chunk {
                uuid: internal_uuid,
                ..
            } = items[0]
            {
                internal_uuid
            } else {
                unimplemented!();
            }
        } else {
            unimplemented!();
        };

        clone.decode_as_message(uuid);
        clone.decode_as_message(internal_uuid.clone());

        if let DecodingState::Message { items, .. } = clone {
            assert_variant!(items[0], DecodingState::Message { .. }  );
        } else {
            unimplemented!();
        };
    }

    #[test]
    fn formats_kind_for_unsigned_varint() {
        let vm = DecodingState::Varint {
            field_number: 0,
            value: 0,
            uuid: Uuid::new_v4(),
            kind: KindVarint::Unsigned,
        };

        assert_eq!("<varint unsign>", vm.get_formatted_kind())
    }

    #[test]
    fn formats_kind_for_zigzag_varint() {
        let vm = DecodingState::Varint {
            field_number: 0,
            value: 0,
            uuid: Uuid::new_v4(),
            kind: KindVarint::ZigZag,
        };

        assert_eq!("<varint zigzag>", vm.get_formatted_kind())
    }

    #[test]
    fn formats_kind_for_fixed_32() {
        let vm = DecodingState::Fixed32 {
            field_number: 0,
            uuid: Uuid::new_v4(),
            kind: Kind32::Fixed32,
            buffer: vec![],
        };

        assert_eq!("<fix 32 unsign>", vm.get_formatted_kind())
    }

    #[test]
    fn formats_kind_for_sfixed_32() {
        let vm = DecodingState::Fixed32 {
            field_number: 0,
            uuid: Uuid::new_v4(),
            kind: Kind32::SFixed32,
            buffer: vec![],
        };

        assert_eq!("<fix 32 sign>", vm.get_formatted_kind())
    }

    #[test]
    fn formats_kind_for_float() {
        let vm = DecodingState::Fixed32 {
            field_number: 0,
            uuid: Uuid::new_v4(),
            kind: Kind32::Float,
            buffer: vec![],
        };

        assert_eq!("<fix 32 float>", vm.get_formatted_kind())
    }

    #[test]
    fn formats_kind_for_buffer_32() {
        let vm = DecodingState::Fixed32 {
            field_number: 0,
            uuid: Uuid::new_v4(),
            kind: Kind32::Buffer,
            buffer: vec![],
        };

        assert_eq!("<fix 32 raw>", vm.get_formatted_kind())
    }

    #[test]
    fn formats_kind_for_fixed_64() {
        let vm = DecodingState::Fixed64 {
            field_number: 0,
            uuid: Uuid::new_v4(),
            kind: Kind64::Fixed64,
            buffer: vec![],
        };

        assert_eq!("<fix 64 unsign>", vm.get_formatted_kind())
    }

    #[test]
    fn formats_kind_for_sfixed_64() {
        let vm = DecodingState::Fixed64 {
            field_number: 0,
            uuid: Uuid::new_v4(),
            kind: Kind64::SFixed64,
            buffer: vec![],
        };

        assert_eq!("<fix 64 sign>", vm.get_formatted_kind())
    }

    #[test]
    fn formats_kind_for_double() {
        let vm = DecodingState::Fixed64 {
            field_number: 0,
            uuid: Uuid::new_v4(),
            kind: Kind64::Double,
            buffer: vec![],
        };

        assert_eq!("<fix 64 double>", vm.get_formatted_kind())
    }

    #[test]
    fn formats_kind_for_buffer_64() {
        let vm = DecodingState::Fixed64 {
            field_number: 0,
            uuid: Uuid::new_v4(),
            kind: Kind64::Buffer,
            buffer: vec![],
        };

        assert_eq!("<fix 64 raw>", vm.get_formatted_kind())
    }

    #[test]
    fn does_not_fail_for_just_serialized_type() {
        let mut tb = Ta_Tb::new();
        tb.set_name("Ivan".into());
        tb.set_email("ivan@smirnov.com".into());
        let mut tc = Ta_Tb_Tc::new();
        tc.set_number("10020030040050060072003\n00400500600700800100 100200300400500600700800100 100200300400500600700800100".into());
        tb.tcs.push(tc);
        let mut ta = Ta::new();
        ta.set_tb(tb);
        let buffer = ta.write_to_bytes().unwrap();

        let mut vm = DecodingState::Chunk {
            field_number: 0,
            uuid: Uuid::new_v4(),
            buffer,
        };

        if let DecodingState::Chunk { uuid, .. } = vm {
            vm.decode_as_message(uuid.clone());
        }

        let uuid = if let DecodingState::Message { items, .. } = &vm {
            if let DecodingState::Chunk { uuid, .. } = items[0] {
                uuid
            } else {
                panic!()
            }
        } else {
            panic!()
        };

        vm.decode_as_message(uuid);

        if let DecodingState::Message { items, .. } = vm {
            assert_variant!(items[0], DecodingState::Message {..});
        }
    }
    #[test]
    fn does_not_fail_for_empty_object() {
        let repeated_values = RepeatedValues::new();

        let buffer = repeated_values.write_to_bytes().unwrap();

        let mut vm = DecodingState::Chunk {
            field_number: 0,
            uuid: Uuid::new_v4(),
            buffer,
        };

        if let DecodingState::Chunk { uuid, .. } = vm {
            vm.decode_as_message(uuid.clone());
        }
    }
}
