use crate::{
    binary_proto_object_loader, data::Data, model::File, model::ProtoDec, ui_message::UiMessage,
};
use js_sys::Uint8Array;
use seed::prelude::*;
use wasm_bindgen_futures::JsFuture;

impl ProtoDec {
    pub fn update(&mut self, message: UiMessage, orders: &mut impl Orders<UiMessage>) {
        match message {
            UiMessage::InitialStateDragEnter => {
                if let ProtoDec::InitialState(initial_state) = self {
                    initial_state.drop_zone_active = true;
                }
            }

            UiMessage::InitialStateDragOver => (),

            UiMessage::InitialStateDragLeave => {
                if let ProtoDec::InitialState(initial_state) = self {
                    initial_state.drop_zone_active = false;
                }
            }

            UiMessage::InitialStateDrop(file_list) => {
                if let ProtoDec::InitialState(initial_state) = self {
                    initial_state.drop_zone_active = false;
                    initial_state.uploaded_file = None;

                    let file = file_list.get(0).expect("cant get 0 file");

                    orders.perform_cmd(async move {
                        let array_buffer: JsValue = JsFuture::from(file.array_buffer())
                            .await
                            .expect("read file");

                        let data = Uint8Array::new(&array_buffer);
                        let mut buffer = vec![0; data.length() as usize];
                        data.copy_to(&mut buffer);

                        UiMessage::InitialStateFileRead {
                            file_name: format!("{} - {} bytes", file.name(), file.size()),
                            buffer,
                        }
                    });
                }
            }

            UiMessage::InitialStateFileRead { file_name, buffer } => {
                if let ProtoDec::InitialState(initial_state) = self {
                    initial_state.uploaded_file = Some(File { file_name, buffer })
                }
            }

            UiMessage::ProcessUploadedFile => {
                if let ProtoDec::InitialState(initial_state) = self {
                    if let Some(uploaded_file) = &initial_state.uploaded_file {
                        let buffer = uploaded_file.buffer.clone();
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
