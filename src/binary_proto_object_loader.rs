use crate::{error::EmptyError, test_data::*};
use protobuf::Message;
use std::path::PathBuf;

pub async fn load_test_proto_object() -> Result<Vec<u8>, EmptyError> {
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

pub async fn load_file_from_fs(path: PathBuf) -> Result<Vec<u8>, EmptyError> {
    let buffer = tokio::fs::read(path).await?;
    Ok(buffer)
}
