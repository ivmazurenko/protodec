use crate::kind64::Kind64;

#[test]
fn toggles_itself_in_cycle() {
    let kind = Kind64::Buffer;

    let actual = kind.toggle().toggle().toggle().toggle();

    assert_eq!(actual, kind);
}
