use iced::{button, executor, Application, Button, Command, Element, Row, Settings, Text};
use protobuf::Message;

pub fn main() {
    State::run(Settings {
        flags: InitialParameters {},
        default_font: Some(include_bytes!("./FiraMono-Medium.ttf")),
        ..Settings::default()
    })
}

enum State {
    InitialState { button: button::State },
    FileOpened(Vec<u8>),
}

#[derive(Debug, Clone)]
enum UiMessage {
    OpenFile,
    FileLoaded(Result<Vec<u8>, PbError>),
}

impl Application for State {
    type Executor = executor::Default;
    type Message = UiMessage;
    type Flags = InitialParameters;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            State::InitialState {
                button: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("protobuf decoder")
    }

    fn update(&mut self, message: UiMessage) -> Command<UiMessage> {
        match message {
            UiMessage::OpenFile => {
                if let State::InitialState { .. } = self {
                    Command::perform(load_file(), UiMessage::FileLoaded)
                } else {
                    Command::none()
                }
            }
            UiMessage::FileLoaded(x) => match x {
                Ok(data) => {
                    *self = State::FileOpened(data);
                    Command::none()
                }
                Err(_e) => unimplemented!(),
            },
        }
    }

    fn view(&mut self) -> Element<UiMessage> {
        match self {
            State::InitialState { button } => Button::new(button, Text::new("open file"))
                .on_press(UiMessage::OpenFile)
                .into(),

            State::FileOpened(bytes) => Row::new()
                .padding(8)
                .spacing(8)
                .push(ascii_text)
                .push(hex_text)
                .into(),
        }
    }
}

async fn load_file() -> Result<Vec<u8>, PbError> {
    let mut person = drister::protos::all::Person::new();

    person.set_name("Ivan".into());
    person.set_id(300);
    person.set_email("ivan@smirnov.com".into());

    let mut phone = drister::protos::all::Person_PhoneNumber::new();
    phone.set_number("89069062666".into());
    phone.set_field_type(drister::protos::all::Person_PhoneType::WORK);

    person.phones.push(phone);

    let mut timestamp = protobuf::well_known_types::Timestamp::new();
    timestamp.set_seconds(12_345_678);
    person.set_last_updated(timestamp);

    let bytes = person.write_to_bytes().unwrap();

    tokio::fs::write("serialized.binary", &bytes).await?;

    Ok(bytes)
}

impl From<std::io::Error> for PbError {
    fn from(err: std::io::Error) -> Self {
        println!("error happens: {}", err);
        PbError {}
    }
}

#[derive(Debug, Clone)]
struct PbError {}

#[derive(Default)]
struct InitialParameters {}
