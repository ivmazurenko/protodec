use crate::wire_type::*;

#[test]
fn has_correct_from_implementation() {
    let wire_type: WireType = From::<u128>::from(0);
    assert_eq!(wire_type, WireType::Varint);

    let wire_type: WireType = From::<u128>::from(1);
    assert_eq!(wire_type, WireType::Size64);

    let wire_type: WireType = From::<u128>::from(2);
    assert_eq!(wire_type, WireType::LengthDelimited);

    let wire_type: WireType = From::<u128>::from(3);
    assert_eq!(wire_type, WireType::StartGroup);

    let wire_type: WireType = From::<u128>::from(4);
    assert_eq!(wire_type, WireType::EndGroup);

    let wire_type: WireType = From::<u128>::from(5);
    assert_eq!(wire_type, WireType::Size32);
}
