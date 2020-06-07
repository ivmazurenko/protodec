use crate::{data_view_model::DataViewModel, format, model::ProtoDec, ui_message::UiMessage};
use iced::{text_input, Align, Button, Column, Container, Element, Length, Row, Scrollable, Text};

impl ProtoDec {
    pub fn view(&mut self) -> Element<UiMessage> {
        match self {
            ProtoDec::InitialState { state } => Container::new(
                Column::new()
                    .width(Length::Fill)
                    .align_items(Align::Center)
                    .spacing(40)
                    .push(Text::new("Protobuf Decoder"))
                    .push(Text::new("To open a file use the command line argument, for example a $protodec binary_file"))
                    .push(Row::new().push(
                        text_input::TextInput::new(
                            &mut state.text_input_state,
                            "Array of bytes as integers, like 0, 1, 255",
                            &state.text_input_value,
                            |v| UiMessage::InitialStateInputChanged(v)))
                        .push(Button::new(&mut state.process_byte_array_button_state, Text::new("Open"))
                                  .on_press(UiMessage::ProcessByteArray), )
                    )
                    .push(
                        Button::new(&mut state.open_file_button_state, Text::new("Open example file"))
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
