use string_utils_rs::StringUtils;

#[test]
fn test_number_integer() {
    assert_eq!(true, "1".is_integer());
    assert_eq!(true, "100".is_integer());
    assert_eq!(true, "0".is_integer());
    assert_eq!(true, "-1".is_integer());
    assert_eq!(true, "-1234".is_integer());

    assert_eq!(false, " -1".is_integer());
    assert_eq!(false, " 0".is_integer());
    assert_eq!(false, " 1 ".is_integer());
    assert_eq!(false, "1 ".is_integer());
    assert_eq!(false, "1.0".is_integer());
    assert_eq!(false, "".is_integer());
}

#[test]
fn test_number_decimal() {
    assert_eq!(true, "1.0".is_decimal());
    assert_eq!(true, "123.33".is_decimal());
    assert_eq!(true, "0".is_decimal());
    assert_eq!(true, "0.0".is_decimal());
    assert_eq!(true, "-1".is_decimal());
    assert_eq!(true, "-1.0".is_decimal());
    assert_eq!(true, "-1.123".is_decimal());
    assert_eq!(true, "-.123".is_decimal());
    assert_eq!(true, "1.".is_decimal());
    assert_eq!(true, ".1".is_decimal());

    assert_eq!(false, "-.1".is_decimal());
    assert_eq!(false, "1.1.1".is_decimal());
    assert_eq!(false, "1.1.".is_decimal());
    assert_eq!(false, " 1.0 ".is_decimal());
    assert_eq!(false, " 1.0".is_decimal());
    assert_eq!(false, "1.0 ".is_decimal());
    assert_eq!(false, "1,0".is_decimal());
    assert_eq!(false, "-".is_decimal());
    assert_eq!(false, "-1-1".is_decimal());
    assert_eq!(false, "--1".is_decimal());
    assert_eq!(false, "".is_decimal());
}

#[test]
fn test_number_unsigned() {
    assert_eq!(true, "1".is_unsigned());
    assert_eq!(true, "0".is_unsigned());
    assert_eq!(true, "01234".is_unsigned());
    assert_eq!(true, "100".is_unsigned());

    assert_eq!(false, "".is_unsigned());
    assert_eq!(false, "1.0".is_unsigned());
    assert_eq!(false, "-1".is_unsigned());
    assert_eq!(false, "-0".is_unsigned());
    assert_eq!(false, ".0".is_unsigned());
    assert_eq!(false, "0.0".is_unsigned());
    assert_eq!(false, " 1".is_unsigned());
    assert_eq!(false, " 1 ".is_unsigned());
    assert_eq!(false, "1 ".is_unsigned());
}

#[test]
fn test_a() {
    let s = ".123";
    let f: f64 = s.parse().unwrap();
    println!("f: {f}");

    //" -.123 ".parse::<f64>().unwrap();
    "-.123".parse::<f64>().unwrap();
    "1.".parse::<f64>().unwrap();
    ".1".parse::<f64>().unwrap();
    "-1.".parse::<f64>().unwrap();
    "-.1".parse::<f64>().unwrap();
    //".123.0".parse::<f64>().unwrap();
}
