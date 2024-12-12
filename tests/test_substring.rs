use string_utils_rs::StringUtils;

#[test]
fn test_substring() {
    let s = "Hello World!";

    assert_eq!("Hello", s.substring(0, 5));
    assert_eq!("d!", s.substring(10, 10));
    assert_eq!("", s.substring(20, 5));
    assert_eq!("", s.substring(999, 10));
    assert_eq!("World", s.substring(6, 5));

    let s = "Rua das Mercês nº45  Fracção C";
    assert_eq!("Rua das Mercês nº45  Fracção C", s.substring(0, 30));
}

#[test]
fn test_substring_byte() {
    let s = "Hello World!";

    assert_eq!("Hello", s.substring_byte(0, 5));
    assert_eq!("d!", s.substring_byte(10, 10));
    assert_eq!("", s.substring_byte(20, 5));
    assert_eq!("", s.substring_byte(999, 10));
    assert_eq!("World", s.substring_byte(6, 5));

    let s = "Rua das Mercês nº45  Fracção C";
    assert_eq!("Rua das Mercês nº45  Fracç", s.substring_byte(0, 30));
}
