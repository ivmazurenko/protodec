use crate::{
    format,
    model::{DecodingState, InitialState, ProtoDec},
    ui_message::UiMessage,
};
use seed::{prelude::*, *};

impl ProtoDec {
    pub fn view(&self) -> Node<UiMessage> {
        div! {
            header!{
                h1!{"Protobuf Decoder"},
                p!["A graphical tool to parse and analyze Google Protobuf messages without knowing their definition."]
            },
            match self {
                ProtoDec::InitialState(initial_state) => view_initial_state(initial_state),
                ProtoDec::Decoding(decoding_state) => view_decoding_state(decoding_state)
            },
        }
    }
}

pub fn view_initial_state(initial_state: &InitialState) -> Node<UiMessage> {
    div![
        label!["Put array of bytes as integers, for example 10, 8, 74, 111, 104, 110, 32, 68, 111, 101, 16, 30"],
        textarea! {
            attrs! {
                At::Value => initial_state.input,
                At::Rows => 6,
                At::Cols => 80,
            },
            input_ev(Ev::Input, UiMessage::InitialStateInputChanged)
        },
        br![],
        button! {"Open bytes array", ev(Ev::Click, |_| UiMessage::ProcessByteArray),},
        br![],
    ]
}

pub fn view_decoding_state(decoding_state: &DecodingState) -> Node<UiMessage> {
    view_decoding_state_recoursive(decoding_state)
}

pub fn view_decoding_state_recoursive(decoding_state: &DecodingState) -> Node<UiMessage> {
    let field_number = decoding_state.get_formatted_field_number();

    let kind = decoding_state.get_formatted_kind();
    div![
        div![
            div![
                C!["im_inline_block"],
                format!("{} {}", field_number.to_string(), kind.to_string())
            ],
            div![C!["im_inline_block"], view_action_buttons(decoding_state)],
        ],
        div![view_value(decoding_state)]
    ]
}

pub fn view_value(decoding_state: &DecodingState) -> Node<UiMessage> {
    match decoding_state {
        DecodingState::Varint { value, kind, .. } => {
            pre! {kind.get_value_as_string(*value)}
        }

        DecodingState::Fixed64 { buffer, kind, .. } => {
            pre! {kind.get_value_as_string(buffer)}
        }

        DecodingState::Fixed32 { buffer, kind, .. } => {
            pre! {kind.get_value_as_string(buffer)}
        }

        DecodingState::Utf8String { value, .. } => {
            pre! {value}
        }

        DecodingState::Chunk { buffer, .. } => {
            pre! {format::format_as_ascii_and_hex(buffer)}
        }

        DecodingState::Message { items, .. } => {
            let items = items
                .iter()
                .map(|item| li! {view_decoding_state_recoursive(item)});

            ul![C!["im_list_style_none"], items]
        }
    }
}

pub fn view_action_buttons(decoding_state: &DecodingState) -> Node<UiMessage> {
    div![match decoding_state {
        DecodingState::Varint { uuid, .. } => {
            let uuid = uuid.clone();
            div![
                button! {C!["im_button_size"],"Toggle", ev(Ev::Click, move |_| UiMessage::Toggle(uuid))},
            ]
        }

        DecodingState::Fixed64 { uuid, .. } => {
            let uuid = uuid.clone();
            div![
                button! {C!["im_button_size"],"Toggle", ev(Ev::Click, move |_| UiMessage::Toggle(uuid))},
            ]
        }

        DecodingState::Fixed32 { uuid, .. } => {
            let uuid = uuid.clone();
            div![
                button! {C!["im_button_size"],"Toggle", ev(Ev::Click, move |_| UiMessage::Toggle(uuid))},
            ]
        }

        DecodingState::Utf8String { .. } => {
            div![]
        }

        DecodingState::Chunk { uuid, .. } => {
            let uuid = uuid.clone();

            div![
                button! {C!["im_button_size"], "Message", ev(Ev::Click, move |_| UiMessage::DecodeChunkAsMessage(uuid)) },
                button! {C!["im_button_size"], "UTF-8", ev(Ev::Click, move |_| UiMessage::DecodeChunkAsUtf8String(uuid)) },
            ]
        }

        DecodingState::Message { .. } => {
            div![]
        }
    }]
}
