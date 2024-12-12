use string_utils_rs::StringUtils;

#[test]
fn test_truncate() {
    let s_orig = "Rua das Mercês nº45  Fracção C";
    let s = s_orig.truncate_byte(30);

    assert_eq!(s_orig.bytes().len(), 34);
    assert_eq!(s, "Rua das Mercês nº45  Fracç");
    assert_eq!(s.len(), 29);

    let s = "Hello World!";

    assert_eq!("Hello", s.truncate_byte(5));
    assert_eq!("Hello World!", s.truncate_byte(999));
    assert_eq!("H", s.truncate_byte(1));
    assert_eq!("", s.truncate_byte(0));
}
