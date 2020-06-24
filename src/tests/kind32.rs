use crate::kind32::Kind32;

#[test]
fn toggles_itself_in_cycle() {
    let kind = Kind32::Buffer;

    let actual = kind.toggle().toggle().toggle().toggle();

    assert_eq!(actual, kind);
}
