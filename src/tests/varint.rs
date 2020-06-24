#[test]
fn correctly_takes_varint_300() {
    let buffer = [0b_1010_1100, 0b_0000_0010];

    let actual = crate::varint::take_varint(&buffer);

    assert_eq!(300, actual.0);
    assert_eq!(0, actual.1.len());
}

#[test]
fn correctly_takes_varint_1() {
    let buffer = [0b_0001];

    let actual = crate::varint::take_varint(&buffer);

    assert_eq!(1, actual.0);
    assert_eq!(0, actual.1.len());
}

#[test]
fn returns_correct_remaining_tail() {
    let buffer = [0b_0001, 0, 0, 0, 0];

    let actual = crate::varint::take_varint(&buffer);

    assert_eq!(4, actual.1.len());
}

#[test]
fn returns_correct_remaining_tail_2() {
    let buffer = [0b_0001, 255, 128, 64, 32];

    let actual = crate::varint::take_varint(&buffer);

    assert_eq!(4, actual.1.len());
}
