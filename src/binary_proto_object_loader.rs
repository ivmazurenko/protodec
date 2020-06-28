use crate::error::ProtodecError;
use std::num::ParseIntError;

pub fn load_test_proto_object() -> Result<Vec<u8>, ProtodecError> {
    panic!();
}

pub fn parse_u8_vec_from_string_with_integers<T: Into<String>>(
    source: T,
) -> Result<std::vec::Vec<u8>, ParseIntError> {
    let source = source.into();
    let mut source = source.trim_end().to_string();

    if let Some(ch) = source.chars().last() {
        if ch == ',' {
            let source_without_last_comma = &source[..(source.len() - 1)];
            source = source_without_last_comma.to_string();
        }
    }

    source
        .replace(' ', "")
        .replace('\n', "")
        .replace('\t', "")
        .split(',')
        .map(|x| x.parse::<u8>())
        .collect()
}
