use crate::{
    format,
    model::{DecodingState, InitialState, ProtoDec},
    ui_message::UiMessage,
};
use seed::{prelude::*, *};

impl ProtoDec {
    pub fn view(&self) -> Node<UiMessage> {
        div! {
            h1!{"Protobuf Decoder"},
            match self {
                ProtoDec::InitialState(initial_state) => view_initial_state(initial_state),
                ProtoDec::Decoding(decoding_state) => view_decoding_state(decoding_state)
            }
        }
    }
}

pub fn view_initial_state(initial_state: &InitialState) -> Node<UiMessage> {
    div![
        label!["Put array of bytes as integers, like 0, 1, 255:"],
        textarea! {
            attrs! {
                At::Value => initial_state.input,
                At::Rows => 6,
                At::Cols => 80,
            },
            input_ev(Ev::Input, UiMessage::InitialStateInputChanged)
        },
        br![],
        button! {"next", ev(Ev::Click, |_| UiMessage::ProcessByteArray),},
        br![],
        label!["or"],
        br![],
        button! {"open debugging file", ev(Ev::Click, |_| UiMessage::OpenFile),},
    ]
}

pub fn view_decoding_state(decoding_state: &DecodingState) -> Node<UiMessage> {
    view_decoding_state_recoursive(decoding_state)
}

pub fn view_decoding_state_recoursive(decoding_state: &DecodingState) -> Node<UiMessage> {
    let field_number = decoding_state.get_formatted_field_number();

    let kind = decoding_state.get_formatted_kind();
    div![
        div![format!("{} {}", field_number.to_string(), kind.to_string())],
        match decoding_state {
            DecodingState::Varint {
                value, uuid, kind, ..
            } => {
                let uuid = uuid.clone();
                div![
                    div![pre! {kind.get_value_as_string(*value)}],
                    div![button! {"Toggle", ev(Ev::Click, move |_| UiMessage::Toggle(uuid)) },],
                ]
            }

            DecodingState::Fixed64 {
                buffer, uuid, kind, ..
            } => {
                let uuid = uuid.clone();
                div![
                    div![pre! {kind.get_value_as_string(buffer)}],
                    div![button! {"Toggle", ev(Ev::Click, move |_| UiMessage::Toggle(uuid)) },],
                ]
            }

            DecodingState::Fixed32 {
                buffer, uuid, kind, ..
            } => {
                let uuid = uuid.clone();
                div![
                    div![pre! {kind.get_value_as_string(buffer)}],
                    div![button! {"Toggle", ev(Ev::Click, move |_| UiMessage::Toggle(uuid)) },],
                ]
            }

            DecodingState::Utf8String { value, .. } => {
                div![pre! {value}]
            }

            DecodingState::Chunk { buffer, uuid, .. } => {
                let uuid = uuid.clone();

                let value_text = format::format_as_ascii_and_hex(buffer);

                div![
                    div![pre! {value_text}],
                    div![
                        button! {"MESG", ev(Ev::Click, move |_| UiMessage::DecodeChunkAsMessage(uuid)) },
                    ],
                    div![
                        button! {"UTF8", ev(Ev::Click, move |_| UiMessage::DecodeChunkAsUtf8String(uuid)) },
                    ],
                ]
            }

            DecodingState::Message { items, .. } => {
                let items = items
                    .iter()
                    .map(|item| li! {view_decoding_state_recoursive(item)});

                ul![items]
            }
        }
    ]
}
