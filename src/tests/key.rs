use crate::tests::test_data::*;
use crate::wire_type::WireType;
use protobuf::Message;

#[test]
fn takes_correct_first_key_for_simple_i32_wrapper() {
    let mut i32_wrapper = I32Wrapper::new();
    i32_wrapper.set_value(117);
    let buffer = i32_wrapper.write_to_bytes().unwrap();

    let (key, _buffer) = crate::key::take_key(&buffer);

    assert_eq!(WireType::Varint, key.wire_type);
    assert_eq!(1, key.field_number);
}

#[test]
fn takes_correct_first_key_for_simple_string_wrapper() {
    let mut string_wrapper = StringWrapper::new();
    string_wrapper.set_value("117".into());
    let buffer = string_wrapper.write_to_bytes().unwrap();

    let (key, _buffer) = crate::key::take_key(&buffer);

    assert_eq!(WireType::LengthDelimited, key.wire_type);
    assert_eq!(1, key.field_number);
}

#[test]
fn takes_correct_keys_for_phone_number() {
    let mut phone_number = Person_PhoneNumber::new();
    phone_number.set_number("117".into());
    phone_number.set_field_type(Person_PhoneType::HOME);

    let buffer = phone_number.write_to_bytes().unwrap();

    let (key, _buffer) = crate::key::take_key(&buffer);

    assert_eq!(WireType::LengthDelimited, key.wire_type);
    assert_eq!(1, key.field_number);

    let (key, _buffer) = crate::key::take_key(&buffer[5..]);

    assert_eq!(WireType::Varint, key.wire_type);
    assert_eq!(2, key.field_number);
}
