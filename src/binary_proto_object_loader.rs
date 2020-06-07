use crate::{error::ProtodecError, test_data::*};
use protobuf::Message;
use std::path::PathBuf;

pub async fn load_test_proto_object() -> Result<Vec<u8>, ProtodecError> {
    let mut person = Person::new();

    person.set_name("Ivan".into());
    person.set_id(300);
    person.set_email("ivan@smirnov.com".into());
    let mut phone = Person_PhoneNumber::new();
    phone.set_number("89069062666".into());
    phone.set_field_type(Person_PhoneType::HOME);
    person.phones.push(phone);

    let mut phone2 = Person_PhoneNumber::new();
    phone2.set_number("89039032672".into());
    phone2.set_field_type(Person_PhoneType::WORK);
    person.phones.push(phone2);

    let mut phone3 = Person_PhoneNumber::new();
    phone3.set_number("100200300400500600700800100".into());
    phone3.set_field_type(Person_PhoneType::MOBILE);
    person.phones.push(phone3);

    let mut timestamp = protobuf::well_known_types::Timestamp::new();
    timestamp.set_seconds(12_345_678);
    person.set_last_updated(timestamp);

    let mut address_book = AddressBook::new();
    address_book.people.push(person);

    let mut different_numbers_wrapper = DifferentNumbersWrapper::new();
    different_numbers_wrapper.set_value_double(1.0);
    different_numbers_wrapper.set_value_int32(2);
    different_numbers_wrapper.set_value_int64(3);
    different_numbers_wrapper.set_value_uint32(4);
    different_numbers_wrapper.set_value_uint64(5);
    different_numbers_wrapper.set_value_sint32(6);
    different_numbers_wrapper.set_value_sint64(7);
    different_numbers_wrapper.set_value_fixed32(8);
    different_numbers_wrapper.set_value_fixed64(9);
    different_numbers_wrapper.set_value_sfixed32(10);
    different_numbers_wrapper.set_value_sfixed64(11);

    let mut big_object = BigObject::new();
    big_object.set_address_book(address_book);
    big_object.set_different_numbers_wrapper(different_numbers_wrapper);

    let buffer = big_object.write_to_bytes().unwrap();

    tokio::fs::write("serialized_proto_object", &buffer).await?;

    Ok(buffer)
}

pub async fn load_file_from_fs(path: PathBuf) -> Result<Vec<u8>, ProtodecError> {
    let buffer = tokio::fs::read(path).await?;
    Ok(buffer)
}

use std::num::ParseIntError;

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
        .split(",")
        .map(|x| x.parse::<u8>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn parses_numbers() {
        let source = "0, 1,  254,255 ";

        let actual = parse_u8_vec_from_string_with_integers(source).unwrap();

        assert_eq!(actual[0], 0_u8);
        assert_eq!(actual[1], 1_u8);
        assert_eq!(actual[2], 254_u8);
        assert_eq!(actual[3], 255_u8);
    }

    #[test]
    fn parses_numbers_from_multiline_string() {
        let source = "0     ,
                      1,
                      254,
                      255 ";

        let actual = parse_u8_vec_from_string_with_integers(source).unwrap();

        assert_eq!(actual[0], 0_u8);
        assert_eq!(actual[1], 1_u8);
        assert_eq!(actual[2], 254_u8);
        assert_eq!(actual[3], 255_u8);
    }

    #[test]
    fn parses_numbers_from_tab_separeted_string() {
        let source = "\t0\t,
                      \t1,\t
                      254\t,
                      255 \t\t";

        let actual = parse_u8_vec_from_string_with_integers(source).unwrap();

        assert_eq!(actual[0], 0_u8);
        assert_eq!(actual[1], 1_u8);
        assert_eq!(actual[2], 254_u8);
        assert_eq!(actual[3], 255_u8);
    }

    #[test]
    fn does_not_parses_invalid_values() {
        assert!(parse_u8_vec_from_string_with_integers("0, 1,  0f,0a ").is_err());
        assert!(parse_u8_vec_from_string_with_integers("0, 1,  254, 256 ").is_err());
        assert!(parse_u8_vec_from_string_with_integers("0 1,  254, 256 ").is_err());
        assert!(parse_u8_vec_from_string_with_integers("-1, 1").is_err());
    }

    #[test]
    fn parses_real_object() {
        let string =
            "10, 101, 10, 99, 10, 4, 73, 118, 97, 110, 16, 172, 2, 26, 16, 105, 118, 97, 110, 64,
            115, 109, 105, 114, 110, 111, 118, 46, 99, 111, 109, 34, 15, 10, 11, 56, 57, 48, 54,
            57, 48, 54, 50, 54, 54, 54, 16, 1, 34, 15, 10, 11, 56, 57, 48, 51, 57, 48, 51, 50, 54,
            55, 50, 16, 2, 34, 29, 10, 27, 49, 48, 48, 50, 48, 48, 51, 48, 48, 52, 48, 48, 53, 48,
            48, 54, 48, 48, 55, 48, 48, 56, 48, 48, 49, 48, 48, 42, 5, 8, 206, 194, 241, 5, 18, 49,
            9, 0, 0, 0, 0, 0, 0, 240, 63, 16, 2, 24, 3, 32, 4, 40, 5, 48, 12, 56, 14, 69, 8, 0, 0,
            0, 73, 9, 0, 0, 0, 0, 0, 0, 0, 85, 10, 0, 0, 0, 89, 11, 0, 0, 0, 0, 0, 0, 0,
        ";

        let actual = parse_u8_vec_from_string_with_integers(string);

        assert_variant!(actual, Ok(_));
    }
}
