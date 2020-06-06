use iced::{
    button, executor, scrollable, Align, Application, Button, Column, Command, Container, Element,
    Length, Row, Scrollable, Settings, Text,
};
use protodec::{
    binary_proto_object_loader, data::Data, data_view_model::DataViewModel, error::ProtodecError,
    format,
};
use std::path::PathBuf;
use uuid::Uuid;

pub fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    let second_arg = args.first();

    let binary_file_path = match second_arg {
        None => None,
        Some(path) => Some(PathBuf::from(path)),
    };

    ProtoDec::run(Settings {
        default_font: Some(include_bytes!("./FiraMono-Medium.ttf")),
        flags: ProtoDecFlags {
            binary_file_path: binary_file_path,
        },
        ..Settings::default()
    });
}

enum ProtoDec {
    InitialState {
        open_file_button_state: button::State,
    },
    Decoding {
        scroll: scrollable::State,
        data_view_model: DataViewModel,
    },
}

#[derive(Default)]
struct ProtoDecFlags {
    binary_file_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
enum UiMessage {
    OpenFile,
    FileLoaded(Result<Vec<u8>, ProtodecError>),
    DecodeChunkAsMessage(Uuid),
    DecodeChunkAsUtf8String(Uuid),
    Toggle(Uuid),
}

impl Application for ProtoDec {
    type Executor = executor::Default;
    type Message = UiMessage;
    type Flags = ProtoDecFlags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        match flags.binary_file_path {
            Some(path) => (
                ProtoDec::InitialState {
                    open_file_button_state: button::State::new(),
                },
                Command::perform(
                    binary_proto_object_loader::load_file_from_fs(path),
                    UiMessage::FileLoaded,
                ),
            ),
            None => (
                ProtoDec::InitialState {
                    open_file_button_state: button::State::new(),
                },
                Command::none(),
            ),
        }
    }

    fn title(&self) -> String {
        String::from("Protobuf Decoder")
    }

    fn update(&mut self, message: UiMessage) -> Command<UiMessage> {
        self.update(message)
    }

    fn view(&mut self) -> Element<UiMessage> {
        self.view()
    }
}

impl ProtoDec {
    fn update(&mut self, message: UiMessage) -> Command<UiMessage> {
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
        }
    }
}

impl ProtoDec {
    fn view(&mut self) -> Element<UiMessage> {
        match self {
            ProtoDec::InitialState {
                open_file_button_state,
            } => Container::new(
                Column::new()
                    .width(Length::Fill)
                    .align_items(Align::Center)
                    .spacing(40)
                    .push(Text::new("Protobuf Decoder"))
                    .push(Text::new("To open a file use the command line argument, for example a $protodec binary_file"))
                    .push(
                        Button::new(open_file_button_state, Text::new("Open example file"))
                            .on_press(UiMessage::OpenFile),
                    ),
            )
            .width(Length::Fill)
            .into(),
            ProtoDec::Decoding {
                data_view_model,
                scroll,
            } => Scrollable::new(scroll)
                .width(Length::Fill)
                .height(Length::Fill)
                .push(view_as_tree(data_view_model))
                .into(),
        }
    }
}

fn view_as_tree(data_view_model: &mut DataViewModel) -> Element<UiMessage> {
    let field_number_text = create_black_text(data_view_model.get_formatted_field_number());
    let kind_text = create_green_text(data_view_model.get_formatted_kind());

    match data_view_model {
        DataViewModel::Varint {
            value,
            button_state,
            uuid,
            kind,
            ..
        } => {
            let value_text = create_blue_text(kind.get_value_as_string(*value));

            let toggle_button = Button::new(button_state, Text::new("Toggle"))
                .on_press(UiMessage::Toggle(*uuid))
                .padding(1);

            Container::new(
                Row::new()
                    .push(field_number_text)
                    .push(kind_text)
                    .push(value_text)
                    .push(toggle_button),
            )
        }

        DataViewModel::Fixed64 {
            buffer,
            uuid,
            button_state,
            kind,
            ..
        } => {
            let value_text = create_blue_text(kind.get_value_as_string(buffer));

            let toggle_button = Button::new(button_state, Text::new("Toggle"))
                .on_press(UiMessage::Toggle(*uuid))
                .padding(1);

            Container::new(
                Row::new()
                    .push(field_number_text)
                    .push(kind_text)
                    .push(value_text)
                    .push(toggle_button),
            )
        }

        DataViewModel::Fixed32 {
            buffer,
            uuid,
            button_state,
            kind,
            ..
        } => {
            let value_text = create_blue_text(kind.get_value_as_string(buffer));

            let toggle_button = Button::new(button_state, Text::new("Toggle"))
                .on_press(UiMessage::Toggle(*uuid))
                .padding(1);

            Container::new(
                Row::new()
                    .push(field_number_text)
                    .push(kind_text)
                    .push(value_text)
                    .push(toggle_button),
            )
        }

        DataViewModel::Chunk {
            buffer,
            uuid,
            decode_as_message_button_state,
            decode_as_utf8_button_state,
            ..
        } => {
            let value_text = create_black_text(format::format_as_ascii_and_hex(buffer));

            let decode_as_message_button =
                Button::new(decode_as_message_button_state, Text::new("Message"))
                    .on_press(UiMessage::DecodeChunkAsMessage(*uuid))
                    .padding(1);
            let decode_as_utf8_string_button =
                Button::new(decode_as_utf8_button_state, Text::new("UTF-8"))
                    .on_press(UiMessage::DecodeChunkAsUtf8String(*uuid))
                    .padding(1);

            Container::new(
                Row::new()
                    .push(field_number_text)
                    .push(kind_text)
                    .push(value_text)
                    .push(decode_as_message_button)
                    .push(decode_as_utf8_string_button),
            )
        }

        DataViewModel::Message { items, .. } => {
            let column = items.iter_mut().fold(
                Column::new().push(Text::new("x").color([1.0, 1.0, 1.0])),
                |x, y| x.push(view_as_tree(y)),
            );

            Container::new(
                Row::new()
                    .push(field_number_text)
                    .push(kind_text)
                    .push(column),
            )
        }

        DataViewModel::Utf8String { value, .. } => {
            let value_text = create_blue_text(value.clone());
            Container::new(
                Row::new()
                    .push(field_number_text)
                    .push(kind_text)
                    .push(value_text),
            )
        }
    }
    .into()
}

fn create_blue_text(value: String) -> Text {
    Text::new(value).color([0.0, 0.0, 0.7])
}

fn create_black_text(value: String) -> Text {
    Text::new(value)
}

fn create_green_text(value: String) -> Text {
    Text::new(value).color([0.0, 0.8, 0.0])
}
