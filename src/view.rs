use crate::{
    format,
    model::{DecodingState, InitialState, ProtoDec},
    ui_message::UiMessage,
};
use seed::{prelude::*, *};
use web_sys::{self};

macro_rules! stop_and_prevent {
    { $event:expr } => {
        {
            $event.stop_propagation();
            $event.prevent_default();
        }
     };
}

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
        label!["Put array of bytes as integers, for example: 10, 8, 74, 111, 104, 110, 32, 68, 111, 101, 16, 30"],
        textarea! {
            attrs! {
                At::Value => initial_state.input,
                At::Rows => 6,
            },
            style![
                St::Width => "100%",
            ],
            input_ev(Ev::Input, UiMessage::InitialStateInputChanged)
        },
        br![],
        button! {"Open bytes array", ev(Ev::Click, |_| UiMessage::ProcessByteArray),},
        br![],
        br![],
        label!["Or in case when you have the binary file:"],
        view_drop_zone(initial_state),
        button! {
            style![
                St::MarginTop => "11px",
            ],
            "Open uploaded file", ev(Ev::Click, |_| UiMessage::ProcessUploadedFile),},
    ]
}

fn view_drop_zone(initial_state: &InitialState) -> Node<UiMessage> {
    div![
        C!["im_drop_zone"],
        style![
            St::Background => if initial_state.drop_zone_active { "#ffffff" } else { "#f6f8fa" },
        ],
        ev(Ev::DragEnter, |event| {
            stop_and_prevent!(event);
            UiMessage::InitialStateDragEnter
        }),
        ev(Ev::DragOver, |event| {
            let drag_event = event.dyn_into::<web_sys::DragEvent>().unwrap();
            stop_and_prevent!(drag_event);
            drag_event.data_transfer().unwrap().set_drop_effect("copy");
            UiMessage::InitialStateDragOver
        }),
        ev(Ev::DragLeave, |event| {
            stop_and_prevent!(event);
            UiMessage::InitialStateDragLeave
        }),
        ev(Ev::Drop, |event| {
            let drag_event = event.dyn_into::<web_sys::DragEvent>().unwrap();
            stop_and_prevent!(drag_event);
            let file_list = drag_event.data_transfer().unwrap().files().unwrap();
            UiMessage::InitialStateDrop(file_list)
        }),
        div![
            style! {
                St::PointerEvents => "none",
            },
            if let Some(uploaded_file) = &initial_state.uploaded_file {
                uploaded_file.file_name.clone()
            } else {
                "Drop your protobuf file here".into()
            }
        ],
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
            div![C!["im_inline_block"], format!("{} {}", field_number, kind)],
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
            let uuid = *uuid;
            div![
                button! {C!["im_button_size"],"Toggle", ev(Ev::Click, move |_| UiMessage::Toggle(uuid))},
            ]
        }

        DecodingState::Fixed64 { uuid, .. } => {
            let uuid = *uuid;
            div![
                button! {C!["im_button_size"],"Toggle", ev(Ev::Click, move |_| UiMessage::Toggle(uuid))},
            ]
        }

        DecodingState::Fixed32 { uuid, .. } => {
            let uuid = *uuid;
            div![
                button! {C!["im_button_size"],"Toggle", ev(Ev::Click, move |_| UiMessage::Toggle(uuid))},
            ]
        }

        DecodingState::Utf8String { .. } => {
            empty()
        }

        DecodingState::Chunk { uuid, .. } => {
            let uuid = *uuid;

            div![
                button! {C!["im_button_size"], "Message", ev(Ev::Click, move |_| UiMessage::DecodeChunkAsMessage(uuid)) },
                button! {C!["im_button_size"], "UTF-8", ev(Ev::Click, move |_| UiMessage::DecodeChunkAsUtf8String(uuid)) },
            ]
        }

        DecodingState::Message { .. } => {
            empty()
        }
    }]
}
