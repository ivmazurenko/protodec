pub mod binary_proto_object_loader;
pub mod data;
pub mod decode;
pub mod error;
pub mod format;
pub mod key;
pub mod kind32;
pub mod kind64;
pub mod kind_varint;
pub mod model;
pub mod test_data;
pub mod ui_message;
pub mod update;
pub mod varint;
pub mod view;
pub mod wire_type;

#[macro_export]
macro_rules! assert_variant {
    ($value:expr, $pattern:pat) => {{
        let value = &$value;

        if let $pattern = value {
        } else {
            panic!(
                r#"assertion failed (value doesn't match pattern):
    value: `{:?}`,
    pattern: `{}`"#,
                value,
                stringify!($pattern)
            )
        }
    }};
}
