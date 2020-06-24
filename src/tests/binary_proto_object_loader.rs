use crate::assert_variant;

#[test]
fn parses_numbers() {
    let source = "0, 1,  254,255 ";

    let actual =
        crate::binary_proto_object_loader::parse_u8_vec_from_string_with_integers(source).unwrap();

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

    let actual =
        crate::binary_proto_object_loader::parse_u8_vec_from_string_with_integers(source).unwrap();

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

    let actual =
        crate::binary_proto_object_loader::parse_u8_vec_from_string_with_integers(source).unwrap();

    assert_eq!(actual[0], 0_u8);
    assert_eq!(actual[1], 1_u8);
    assert_eq!(actual[2], 254_u8);
    assert_eq!(actual[3], 255_u8);
}

#[test]
fn does_not_parses_invalid_values() {
    assert!(
        crate::binary_proto_object_loader::parse_u8_vec_from_string_with_integers("0, 1,  0f,0a ")
            .is_err()
    );
    assert!(
        crate::binary_proto_object_loader::parse_u8_vec_from_string_with_integers(
            "0, 1,  254, 256 "
        )
        .is_err()
    );
    assert!(
        crate::binary_proto_object_loader::parse_u8_vec_from_string_with_integers(
            "0 1,  254, 256 "
        )
        .is_err()
    );
    assert!(
        crate::binary_proto_object_loader::parse_u8_vec_from_string_with_integers("-1, 1").is_err()
    );
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

    let actual = crate::binary_proto_object_loader::parse_u8_vec_from_string_with_integers(string);

    assert_variant!(actual, Ok(_));
}
