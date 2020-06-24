use crate::model::*;
use crate::tests::test_data::*;
use crate::*;
use crate::{kind32::Kind32, kind64::Kind64, kind_varint::KindVarint};
use protobuf::Message;
use uuid::Uuid;

#[test]
fn converts_itself_into_message() {
    let mut string_wrapper = StringWrapper::new();
    string_wrapper.set_value("117".into());
    let buffer = string_wrapper.write_to_bytes().unwrap();

    let uuid = Uuid::new_v4();

    let mut state = DecodingState::Chunk {
        buffer,
        uuid,
        field_number: 123,
    };

    state.decode_as_message(uuid);
    assert_variant!(state, DecodingState::Message{..});
}

#[test]
fn converts_internal_chunk_to_message() {
    let mut string_wrapper = StringWrapper::new();
    string_wrapper.set_value("117".into());
    let mut string_wrapper_wrapper = StringWrapperWrapper::new();
    string_wrapper_wrapper.set_value(string_wrapper);

    let buffer = string_wrapper_wrapper.write_to_bytes().unwrap();

    let uuid = Uuid::new_v4();

    let mut state = DecodingState::Chunk {
        buffer,
        uuid,
        field_number: 123,
    };

    state.decode_as_message(uuid);
    let mut clone = state.clone();

    let internal_uuid = if let DecodingState::Message { items, .. } = state {
        if let DecodingState::Chunk {
            uuid: internal_uuid,
            ..
        } = items[0]
        {
            internal_uuid
        } else {
            unimplemented!();
        }
    } else {
        unimplemented!();
    };

    clone.decode_as_message(uuid);
    clone.decode_as_message(internal_uuid.clone());

    if let DecodingState::Message { items, .. } = clone {
        assert_variant!(items[0], DecodingState::Message { .. }  );
    } else {
        unimplemented!();
    };
}

#[test]
fn formats_kind_for_unsigned_varint() {
    let vm = DecodingState::Varint {
        field_number: 0,
        value: 0,
        uuid: Uuid::new_v4(),
        kind: KindVarint::Unsigned,
    };

    assert_eq!("<varint unsign>", vm.get_formatted_kind())
}

#[test]
fn formats_kind_for_zigzag_varint() {
    let vm = DecodingState::Varint {
        field_number: 0,
        value: 0,
        uuid: Uuid::new_v4(),
        kind: KindVarint::ZigZag,
    };

    assert_eq!("<varint zigzag>", vm.get_formatted_kind())
}

#[test]
fn formats_kind_for_fixed_32() {
    let vm = DecodingState::Fixed32 {
        field_number: 0,
        uuid: Uuid::new_v4(),
        kind: Kind32::Fixed32,
        buffer: vec![],
    };

    assert_eq!("<fix 32 unsign>", vm.get_formatted_kind())
}

#[test]
fn formats_kind_for_sfixed_32() {
    let vm = DecodingState::Fixed32 {
        field_number: 0,
        uuid: Uuid::new_v4(),
        kind: Kind32::SFixed32,
        buffer: vec![],
    };

    assert_eq!("<fix 32 sign>", vm.get_formatted_kind())
}

#[test]
fn formats_kind_for_float() {
    let vm = DecodingState::Fixed32 {
        field_number: 0,
        uuid: Uuid::new_v4(),
        kind: Kind32::Float,
        buffer: vec![],
    };

    assert_eq!("<fix 32 float>", vm.get_formatted_kind())
}

#[test]
fn formats_kind_for_buffer_32() {
    let vm = DecodingState::Fixed32 {
        field_number: 0,
        uuid: Uuid::new_v4(),
        kind: Kind32::Buffer,
        buffer: vec![],
    };

    assert_eq!("<fix 32 raw>", vm.get_formatted_kind())
}

#[test]
fn formats_kind_for_fixed_64() {
    let vm = DecodingState::Fixed64 {
        field_number: 0,
        uuid: Uuid::new_v4(),
        kind: Kind64::Fixed64,
        buffer: vec![],
    };

    assert_eq!("<fix 64 unsign>", vm.get_formatted_kind())
}

#[test]
fn formats_kind_for_sfixed_64() {
    let vm = DecodingState::Fixed64 {
        field_number: 0,
        uuid: Uuid::new_v4(),
        kind: Kind64::SFixed64,
        buffer: vec![],
    };

    assert_eq!("<fix 64 sign>", vm.get_formatted_kind())
}

#[test]
fn formats_kind_for_double() {
    let vm = DecodingState::Fixed64 {
        field_number: 0,
        uuid: Uuid::new_v4(),
        kind: Kind64::Double,
        buffer: vec![],
    };

    assert_eq!("<fix 64 double>", vm.get_formatted_kind())
}

#[test]
fn formats_kind_for_buffer_64() {
    let vm = DecodingState::Fixed64 {
        field_number: 0,
        uuid: Uuid::new_v4(),
        kind: Kind64::Buffer,
        buffer: vec![],
    };

    assert_eq!("<fix 64 raw>", vm.get_formatted_kind())
}

#[test]
fn does_not_fail_for_just_serialized_type() {
    let mut tb = Ta_Tb::new();
    tb.set_name("Ivan".into());
    tb.set_email("ivan@smirnov.com".into());
    let mut tc = Ta_Tb_Tc::new();
    tc.set_number("10020030040050060072003\n00400500600700800100 100200300400500600700800100 100200300400500600700800100".into());
    tb.tcs.push(tc);
    let mut ta = Ta::new();
    ta.set_tb(tb);
    let buffer = ta.write_to_bytes().unwrap();

    let mut vm = DecodingState::Chunk {
        field_number: 0,
        uuid: Uuid::new_v4(),
        buffer,
    };

    if let DecodingState::Chunk { uuid, .. } = vm {
        vm.decode_as_message(uuid.clone());
    }

    let uuid = if let DecodingState::Message { items, .. } = &vm {
        if let DecodingState::Chunk { uuid, .. } = items[0] {
            uuid
        } else {
            panic!()
        }
    } else {
        panic!()
    };

    vm.decode_as_message(uuid);

    if let DecodingState::Message { items, .. } = vm {
        assert_variant!(items[0], DecodingState::Message {..});
    }
}
#[test]
fn does_not_fail_for_empty_object() {
    let repeated_values = RepeatedValues::new();

    let buffer = repeated_values.write_to_bytes().unwrap();

    let mut vm = DecodingState::Chunk {
        field_number: 0,
        uuid: Uuid::new_v4(),
        buffer,
    };

    if let DecodingState::Chunk { uuid, .. } = vm {
        vm.decode_as_message(uuid.clone());
    }
}
