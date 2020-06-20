use crate::{data, data::Data, error::ProtodecError, key};

pub fn decode_message(buffer: &[u8]) -> Result<Vec<Data>, ProtodecError> {
    let mut result: Vec<Data> = vec![];

    if buffer.len() == 0 {
        return Ok(result);
    }

    let mut buf = buffer;

    loop {
        let (key, rest) = key::take_key(buf);

        let (data, rest) = data::take_data(key, rest)?;

        buf = rest;

        result.push(data);

        if rest.is_empty() {
            break;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_variant;
    use crate::test_data::*;
    use protobuf::Message;

    #[test]
    fn parses_wire_types_of_simple_i32_wrapper() {
        let mut i32_wrapper = I32Wrapper::new();
        i32_wrapper.set_value(117);
        let buffer = i32_wrapper.write_to_bytes().unwrap();

        let actual = decode_message(&buffer).unwrap();

        assert!(actual.len() == 1);
        assert_variant!(actual[0], Data::Varint{..});
    }

    #[test]
    fn parses_wire_types_of_simple_string_wrapper() {
        let mut string_wrapper = StringWrapper::new();
        string_wrapper.set_value("117".into());
        let buffer = string_wrapper.write_to_bytes().unwrap();

        let actual = decode_message(&buffer).unwrap();

        assert!(actual.len() == 1);
        assert_variant!(actual[0], Data::Chunk{..});
    }

    #[test]
    fn parses_wire_types_of_full_phone_type() {
        let mut phone_number = Person_PhoneNumber::new();
        phone_number.set_number("117".into());
        phone_number.set_field_type(Person_PhoneType::HOME);

        let buffer = phone_number.write_to_bytes().unwrap();

        let actual = decode_message(&buffer).unwrap();

        assert!(actual.len() == 2);
        assert_variant!(actual[0], Data::Chunk{..});
        assert_variant!(actual[1], Data::Varint{..});
    }

    #[test]
    fn parses_wire_types_of_person() {
        let mut person = Person::new();

        person.set_name("Ivan".into());
        person.set_id(300);
        person.set_email("ivan@smirnov.com".into());
        let mut phone = Person_PhoneNumber::new();
        phone.set_number("89069062666".into());
        phone.set_field_type(Person_PhoneType::WORK);
        person.phones.push(phone);
        let mut timestamp = protobuf::well_known_types::Timestamp::new();
        timestamp.set_seconds(12_345_678);
        person.set_last_updated(timestamp);

        let buffer = person.write_to_bytes().unwrap();

        let actual = decode_message(&buffer).unwrap();

        assert!(actual.len() == 5);
        assert_variant!(actual[0], Data::Chunk{..});
        assert_variant!(actual[1], Data::Varint{..});
        assert_variant!(actual[2], Data::Chunk{..});
        assert_variant!(actual[3], Data::Chunk{..});
        assert_variant!(actual[4], Data::Chunk{..});
    }

    #[test]
    fn parses_number_of_full_phone_type() {
        let mut phone_number = Person_PhoneNumber::new();
        phone_number.set_number("117".into());
        phone_number.set_field_type(Person_PhoneType::HOME);

        let buffer = phone_number.write_to_bytes().unwrap();

        let actual = decode_message(&buffer).unwrap();

        if let Data::Chunk {
            buffer,
            field_number: 1,
        } = &actual[0]
        {
            assert_eq!(3, buffer.len());
            assert_eq!("117", String::from_utf8(buffer.to_vec()).unwrap())
        } else {
            panic!()
        }
    }

    #[test]
    fn parses_wire_types_of_different_numbers_wrapper() {
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

        let buffer = different_numbers_wrapper.write_to_bytes().unwrap();

        let actual = decode_message(&buffer).unwrap();

        assert!(actual.len() == 11);
        assert_variant!(actual[0], Data::Fixed64{..});
        assert_variant!(actual[1], Data::Varint{..});
        assert_variant!(actual[2], Data::Varint{..});
        assert_variant!(actual[3], Data::Varint{..});
        assert_variant!(actual[4], Data::Varint{..});
        assert_variant!(actual[5], Data::Varint{..});
        assert_variant!(actual[6], Data::Varint{..});
        assert_variant!(actual[7], Data::Fixed32{..});
        assert_variant!(actual[8], Data::Fixed64{..});
        assert_variant!(actual[9], Data::Fixed32{..});
        assert_variant!(actual[10], Data::Fixed64{..});
    }

    #[test]
    fn parses_and_decodes_test_data_for_string_with_integers() {
        let buffer: Vec<u8> = vec![
            10, 101, 10, 99, 10, 4, 73, 118, 97, 110, 16, 172, 2, 26, 16, 105, 118, 97, 110, 64,
            115, 109, 105, 114, 110, 111, 118, 46, 99, 111, 109, 34, 15, 10, 11, 56, 57, 48, 54,
            57, 48, 54, 50, 54, 54, 54, 16, 1, 34, 15, 10, 11, 56, 57, 48, 51, 57, 48, 51, 50, 54,
            55, 50, 16, 2, 34, 29, 10, 27, 49, 48, 48, 50, 48, 48, 51, 48, 48, 52, 48, 48, 53, 48,
            48, 54, 48, 48, 55, 48, 48, 56, 48, 48, 49, 48, 48, 42, 5, 8, 206, 194, 241, 5, 18, 49,
            9, 0, 0, 0, 0, 0, 0, 240, 63, 16, 2, 24, 3, 32, 4, 40, 5, 48, 12, 56, 14, 69, 8, 0, 0,
            0, 73, 9, 0, 0, 0, 0, 0, 0, 0, 85, 10, 0, 0, 0, 89, 11, 0, 0, 0, 0, 0, 0, 0,
        ];

        let actual = decode_message(&buffer).unwrap();
        assert_eq!(2, actual.len());
    }
}
