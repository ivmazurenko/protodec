use crate::kind_varint::KindVarint;

#[test]
fn next_works_in_cycle() {
    let kind = KindVarint::ZigZag;

    let actual = kind.toggle().toggle();

    assert_eq!(actual, kind);
}

#[test]
fn decodes_zigzag_correctly_64() {
    assert_eq!("0", KindVarint::ZigZag.get_value_as_string(0));
    assert_eq!("-1", KindVarint::ZigZag.get_value_as_string(1));
    assert_eq!("1", KindVarint::ZigZag.get_value_as_string(2));
    assert_eq!("-2", KindVarint::ZigZag.get_value_as_string(3));
    assert_eq!(
        "2147483647",
        KindVarint::ZigZag.get_value_as_string(4294967294)
    );
    assert_eq!(
        "-2147483648",
        KindVarint::ZigZag.get_value_as_string(4294967295)
    );
}
