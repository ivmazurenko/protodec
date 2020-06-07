use iced::{Application, Settings};
use protodec::{flags::ProtoDecFlags, model::ProtoDec};
use std::path::PathBuf;

pub fn main() {
    let args_except_command = std::env::args().skip(1).collect::<Vec<String>>();
    let second_arg = args_except_command.first();

    let binary_file_path = match second_arg {
        None => None,
        Some(path) => Some(PathBuf::from(path)),
    };

    ProtoDec::run(Settings {
        default_font: Some(include_bytes!("./FiraMono-Medium.ttf")),
        flags: ProtoDecFlags { binary_file_path },
        ..Settings::default()
    });
}
