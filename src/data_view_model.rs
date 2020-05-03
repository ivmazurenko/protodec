use crate::{data::Data, decode, kind32::Kind32, kind64::Kind64, kind_varint::KindVarint};
use iced::button;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum DataViewModel {
    Varint {
        field_number: u128,
        value: u128,
        uuid: Uuid,
        button_state: button::State,
        kind: KindVarint,
    },

    Fixed64 {
        field_number: u128,
        buffer: Vec<u8>,
        uuid: Uuid,
        button_state: button::State,
        kind: Kind64,
    },

    Fixed32 {
        field_number: u128,
        buffer: Vec<u8>,
        uuid: Uuid,
        button_state: button::State,
        kind: Kind32,
    },

    Chunk {
        field_number: u128,
        buffer: Vec<u8>,
        uuid: Uuid,
        decode_as_message_button_state: button::State,
        decode_as_utf8_button_state: button::State,
    },

    Message {
        field_number: u128,
        items: Vec<DataViewModel>,
        uuid: Uuid,
    },

    Utf8String {
        field_number: u128,
        value: String,
        uuid: Uuid,
    },
}

impl From<Data> for DataViewModel {
    fn from(data: Data) -> Self {
        match data {
            Data::Varint {
                field_number,
                value,
            } => DataViewModel::Varint {
                field_number,
                value,
                uuid: Uuid::new_v4(),
                button_state: button::State::new(),
                kind: KindVarint::Unsigned,
            },

            Data::Fixed64 {
                field_number,
                buffer,
            } => DataViewModel::Fixed64 {
                field_number,
                buffer,
                uuid: Uuid::new_v4(),
                button_state: button::State::new(),
                kind: Kind64::Fixed64,
            },

            Data::Fixed32 {
                field_number,
                buffer,
            } => DataViewModel::Fixed32 {
                field_number,
                buffer,
                uuid: Uuid::new_v4(),
                button_state: button::State::new(),
                kind: Kind32::Fixed32,
            },

            Data::Chunk {
                field_number,
                buffer,
            } => DataViewModel::Chunk {
                field_number,
                buffer,
                uuid: Uuid::new_v4(),
                decode_as_message_button_state: button::State::new(),
                decode_as_utf8_button_state: button::State::new(),
            },

            Data::Message {
                field_number,
                items,
            } => DataViewModel::Message {
                field_number,
                items: items.iter().map(|d| Self::from(d.clone())).collect(),
                uuid: Uuid::new_v4(),
            },
        }
    }
}

impl DataViewModel {
    pub fn decode_as_message(&mut self, ui_event_uuid: Uuid) {
        match self {
            DataViewModel::Chunk {
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
            DataViewModel::Message { items, .. } => {
                for inner_data_view_model in items {
                    inner_data_view_model.decode_as_message(ui_event_uuid);
                }
            }
            _ => {}
        }
    }

    pub fn decode_as_utf8_string(&mut self, ui_event_uuid: Uuid) {
        match self {
            DataViewModel::Chunk {
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
            DataViewModel::Message { items, .. } => {
                for inner_data_view_model in items {
                    inner_data_view_model.decode_as_utf8_string(ui_event_uuid);
                }
            }
            _ => {}
        }
    }

    pub fn toggle(&mut self, ui_event_uuid: Uuid) {
        match self {
            DataViewModel::Varint { uuid, kind, .. } => {
                if ui_event_uuid == *uuid {
                    *kind = kind.toggle()
                }
            }

            DataViewModel::Fixed32 { uuid, kind, .. } => {
                if ui_event_uuid == *uuid {
                    *kind = kind.toggle()
                }
            }

            DataViewModel::Fixed64 { uuid, kind, .. } => {
                if ui_event_uuid == *uuid {
                    *kind = kind.toggle()
                }
            }

            DataViewModel::Message { items, .. } => {
                for inner_data_view_model in items {
                    inner_data_view_model.toggle(ui_event_uuid);
                }
            }
            _ => {}
        }
    }

    pub fn get_formatted_field_number(&self) -> String {
        let field_number = match self {
            DataViewModel::Varint { field_number, .. }
            | DataViewModel::Fixed64 { field_number, .. }
            | DataViewModel::Fixed32 { field_number, .. }
            | DataViewModel::Chunk { field_number, .. }
            | DataViewModel::Message { field_number, .. }
            | DataViewModel::Utf8String { field_number, .. } => field_number,
        };

        format!("{:<2?}", field_number)
    }

    pub fn get_formatted_kind(&self) -> String {
        match self {
            DataViewModel::Varint { kind, .. } => format!("<varint {}>", kind.to_string()),
            DataViewModel::Fixed64 { kind, .. } => format!("<fix 64 {}>", kind.to_string()),
            DataViewModel::Fixed32 { kind, .. } => format!("<fix 32 {}>", kind.to_string()),
            DataViewModel::Chunk { .. } => "<chunk>".to_string(),
            DataViewModel::Message { .. } => "<msg>".to_string(),
            DataViewModel::Utf8String { .. } => "<utf-8>".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data::*;
    use crate::*;
    use iced::button;
    use protobuf::Message;

    #[test]
    fn converts_itself_into_message() {
        let mut string_wrapper = StringWrapper::new();
        string_wrapper.set_value("117".into());
        let buffer = string_wrapper.write_to_bytes().unwrap();

        let uuid = Uuid::new_v4();

        let mut data_view_model = DataViewModel::Chunk {
            buffer,
            decode_as_message_button_state: button::State::new(),
            decode_as_utf8_button_state: button::State::new(),
            uuid,
            field_number: 123,
        };

        data_view_model.decode_as_message(uuid);
        assert_variant!(data_view_model, DataViewModel::Message{..});
    }

    #[test]
    fn converts_internal_chunk_to_message() {
        let mut string_wrapper = StringWrapper::new();
        string_wrapper.set_value("117".into());
        let mut string_wrapper_wrapper = StringWrapperWrapper::new();
        string_wrapper_wrapper.set_value(string_wrapper);

        let buffer = string_wrapper_wrapper.write_to_bytes().unwrap();

        let uuid = Uuid::new_v4();

        let mut data_view_model = DataViewModel::Chunk {
            buffer,
            decode_as_message_button_state: button::State::new(),
            decode_as_utf8_button_state: button::State::new(),
            uuid,
            field_number: 123,
        };

        data_view_model.decode_as_message(uuid);
        let mut clone = data_view_model.clone();

        let internal_uuid = if let DataViewModel::Message { items, .. } = data_view_model {
            if let DataViewModel::Chunk {
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

        if let DataViewModel::Message { items, .. } = clone {
            assert_variant!(items[0], DataViewModel::Message { .. }  );
        } else {
            unimplemented!();
        };
    }

    #[test]
    fn formats_kind_for_unsigned_varint() {
        let vm = DataViewModel::Varint {
            field_number: 0,
            value: 0,
            uuid: Uuid::new_v4(),
            button_state: button::State::new(),
            kind: KindVarint::Unsigned,
        };

        assert_eq!("<varint unsign>", vm.get_formatted_kind())
    }

    #[test]
    fn formats_kind_for_zigzag_varint() {
        let vm = DataViewModel::Varint {
            field_number: 0,
            value: 0,
            uuid: Uuid::new_v4(),
            button_state: button::State::new(),
            kind: KindVarint::ZigZag,
        };

        assert_eq!("<varint zigzag>", vm.get_formatted_kind())
    }

    #[test]
    fn formats_kind_for_fixed_32() {
        let vm = DataViewModel::Fixed32 {
            field_number: 0,
            uuid: Uuid::new_v4(),
            button_state: button::State::new(),
            kind: Kind32::Fixed32,
            buffer: vec![],
        };

        assert_eq!("<fix 32 unsign>", vm.get_formatted_kind())
    }

    #[test]
    fn formats_kind_for_sfixed_32() {
        let vm = DataViewModel::Fixed32 {
            field_number: 0,
            uuid: Uuid::new_v4(),
            button_state: button::State::new(),
            kind: Kind32::SFixed32,
            buffer: vec![],
        };

        assert_eq!("<fix 32 sign>", vm.get_formatted_kind())
    }

    #[test]
    fn formats_kind_for_float() {
        let vm = DataViewModel::Fixed32 {
            field_number: 0,
            uuid: Uuid::new_v4(),
            button_state: button::State::new(),
            kind: Kind32::Float,
            buffer: vec![],
        };

        assert_eq!("<fix 32 float>", vm.get_formatted_kind())
    }

    #[test]
    fn formats_kind_for_buffer_32() {
        let vm = DataViewModel::Fixed32 {
            field_number: 0,
            uuid: Uuid::new_v4(),
            button_state: button::State::new(),
            kind: Kind32::Buffer,
            buffer: vec![],
        };

        assert_eq!("<fix 32 raw>", vm.get_formatted_kind())
    }

    #[test]
    fn formats_kind_for_fixed_64() {
        let vm = DataViewModel::Fixed64 {
            field_number: 0,
            uuid: Uuid::new_v4(),
            button_state: button::State::new(),
            kind: Kind64::Fixed64,
            buffer: vec![],
        };

        assert_eq!("<fix 64 unsign>", vm.get_formatted_kind())
    }

    #[test]
    fn formats_kind_for_sfixed_64() {
        let vm = DataViewModel::Fixed64 {
            field_number: 0,
            uuid: Uuid::new_v4(),
            button_state: button::State::new(),
            kind: Kind64::SFixed64,
            buffer: vec![],
        };

        assert_eq!("<fix 64 sign>", vm.get_formatted_kind())
    }

    #[test]
    fn formats_kind_for_double() {
        let vm = DataViewModel::Fixed64 {
            field_number: 0,
            uuid: Uuid::new_v4(),
            button_state: button::State::new(),
            kind: Kind64::Double,
            buffer: vec![],
        };

        assert_eq!("<fix 64 double>", vm.get_formatted_kind())
    }

    #[test]
    fn formats_kind_for_buffer_64() {
        let vm = DataViewModel::Fixed64 {
            field_number: 0,
            uuid: Uuid::new_v4(),
            button_state: button::State::new(),
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

        let mut vm = DataViewModel::Chunk {
            field_number: 0,
            uuid: Uuid::new_v4(),
            decode_as_message_button_state: button::State::new(),
            decode_as_utf8_button_state: button::State::new(),
            buffer,
        };

        if let DataViewModel::Chunk { uuid, .. } = vm {
            vm.decode_as_message(uuid.clone());
        }

        let uuid = if let DataViewModel::Message { items, .. } = &vm {
            if let DataViewModel::Chunk { uuid, .. } = items[0] {
                uuid
            } else {
                panic!()
            }
        } else {
            panic!()
        };

        vm.decode_as_message(uuid);

        if let DataViewModel::Message { items, .. } = vm {
            assert_variant!(items[0], DataViewModel::Message {..});
        }
    }
}
