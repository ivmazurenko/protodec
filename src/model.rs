use crate::{
    binary_proto_object_loader, data_view_model::DataViewModel, flags::ProtoDecFlags,
    ui_message::UiMessage,
};
use iced::{button, executor, scrollable, text_input, Application, Command, Element};

pub enum ProtoDec {
    InitialState {
        state: InitialState,
    },
    Decoding {
        scroll: scrollable::State,
        data_view_model: DataViewModel,
    },
}

#[derive(Default)]
pub struct InitialState {
    pub open_file_button_state: button::State,
    pub process_byte_array_button_state: button::State,
    pub text_input_state: text_input::State,
    pub text_input_value: String,
}

impl Application for ProtoDec {
    type Executor = executor::Default;
    type Message = UiMessage;
    type Flags = ProtoDecFlags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        match flags.binary_file_path {
            Some(path) => (
                ProtoDec::InitialState {
                    state: InitialState::default(),
                },
                Command::perform(
                    binary_proto_object_loader::load_file_from_fs(path),
                    UiMessage::FileLoaded,
                ),
            ),
            None => (
                ProtoDec::InitialState {
                    state: InitialState::default(),
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
