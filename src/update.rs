use crate::{binary_proto_object_loader, data::Data, model::ProtoDec, ui_message::UiMessage};
use iced::{scrollable, Command};

impl ProtoDec {
    pub fn update(&mut self, message: UiMessage) -> Command<UiMessage> {
        match message {
            UiMessage::OpenFile => {
                if let ProtoDec::InitialState { .. } = self {
                    Command::perform(
                        binary_proto_object_loader::load_test_proto_object(),
                        UiMessage::FileLoaded,
                    )
                } else {
                    Command::none()
                }
            }

            UiMessage::FileLoaded(x) => match x {
                Ok(buffer) => {
                    *self = ProtoDec::Decoding {
                        scroll: scrollable::State::new(),
                        data_view_model: Data::Chunk {
                            buffer,
                            field_number: 0,
                        }
                        .into(),
                    };
                    Command::none()
                }
                Err(e) => panic!(e),
            },

            UiMessage::ProcessByteArray => {
                if let ProtoDec::InitialState { state } = self {
                    let buffer = binary_proto_object_loader::parse_u8_vec_from_string_with_integers(
                        state.text_input_value.clone(),
                    );
                    if let Ok(buffer) = buffer {
                        *self = ProtoDec::Decoding {
                            scroll: scrollable::State::new(),
                            data_view_model: Data::Chunk {
                                buffer,
                                field_number: 0,
                            }
                            .into(),
                        };
                    }
                }
                Command::none()
            }

            UiMessage::DecodeChunkAsMessage(uuid) => {
                if let ProtoDec::Decoding {
                    data_view_model, ..
                } = self
                {
                    data_view_model.decode_as_message(uuid);
                }
                Command::none()
            }

            UiMessage::Toggle(uuid) => {
                if let ProtoDec::Decoding {
                    data_view_model, ..
                } = self
                {
                    data_view_model.toggle(uuid);
                }
                Command::none()
            }

            UiMessage::DecodeChunkAsUtf8String(uuid) => {
                if let ProtoDec::Decoding {
                    data_view_model, ..
                } = self
                {
                    data_view_model.decode_as_utf8_string(uuid);
                }
                Command::none()
            }

            UiMessage::InitialStateInputChanged(value) => {
                if let ProtoDec::InitialState { state } = self {
                    state.text_input_value = value
                }
                Command::none()
            }
        }
    }
}
