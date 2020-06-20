use crate::{binary_proto_object_loader, data::Data, model::ProtoDec, ui_message::UiMessage};
use seed::prelude::*;

impl ProtoDec {
    pub fn update(&mut self, message: UiMessage, orders: &mut impl Orders<UiMessage>) {
        match message {
            UiMessage::OpenFile => {
                if let ProtoDec::InitialState(_) = self {
                    orders.send_msg(UiMessage::FileLoaded(
                        binary_proto_object_loader::load_test_proto_object(),
                    ));
                }
            }

            UiMessage::FileLoaded(x) => match x {
                Ok(buffer) => {
                    *self = ProtoDec::Decoding(
                        Data::Chunk {
                            buffer,
                            field_number: 0,
                        }
                        .into(),
                    );
                }
                Err(e) => panic!(e),
            },

            UiMessage::ProcessByteArray => {
                if let ProtoDec::InitialState(initial_state) = self {
                    let buffer = binary_proto_object_loader::parse_u8_vec_from_string_with_integers(
                        initial_state.input.clone(),
                    );
                    if let Ok(buffer) = buffer {
                        *self = ProtoDec::Decoding(
                            Data::Chunk {
                                buffer,
                                field_number: 0,
                            }
                            .into(),
                        );
                    }
                }
            }

            UiMessage::DecodeChunkAsMessage(uuid) => {
                if let ProtoDec::Decoding(decoding_state) = self {
                    decoding_state.decode_as_message(uuid);
                }
            }

            UiMessage::Toggle(uuid) => {
                if let ProtoDec::Decoding(decoding_state) = self {
                    decoding_state.toggle(uuid);
                }
            }

            UiMessage::DecodeChunkAsUtf8String(uuid) => {
                if let ProtoDec::Decoding(decoding_state) = self {
                    decoding_state.decode_as_utf8_string(uuid);
                }
            }

            UiMessage::InitialStateInputChanged(value) => {
                if let ProtoDec::InitialState(initial_state) = self {
                    initial_state.input = value
                }
            }
        }
    }
}
