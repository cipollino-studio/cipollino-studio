
use crate::value_skipped;

#[test]
fn small_string() {

    let small_string_bytes = &[0b10000011, 65, 66, 67];
    let mut decoder = alisa::Decoder::new(small_string_bytes);
    assert!(decoder.is_string());
    assert_eq!(decoder.read_string(), Some("ABC"));
    assert!(value_skipped(small_string_bytes));
    assert_eq!(alisa::parse_abf(small_string_bytes), Some(alisa::ABFValue::Str("ABC".into())));

}

#[test]
fn string_u8() {

    let u8_string_bytes = &[0b11001100, 3, 65, 66, 67];
    let mut decoder = alisa::Decoder::new(u8_string_bytes);
    assert!(decoder.is_string());
    assert_eq!(decoder.read_string(), Some("ABC"));
    assert!(value_skipped(u8_string_bytes));
    assert_eq!(alisa::parse_abf(u8_string_bytes), Some(alisa::ABFValue::Str("ABC".into())));

}

#[test]
fn string_u16() {

    let u16_string_bytes = &[0b11001101, 3, 0, 65, 66, 67];
    let mut decoder = alisa::Decoder::new(u16_string_bytes);
    assert!(decoder.is_string());
    assert_eq!(decoder.read_string(), Some("ABC"));
    assert!(value_skipped(u16_string_bytes));
    assert_eq!(alisa::parse_abf(u16_string_bytes), Some(alisa::ABFValue::Str("ABC".into())));


}

#[test]
fn string_u32() {

    let u32_string_bytes = &[0b11001110, 3, 0, 0, 0, 65, 66, 67];
    let mut decoder = alisa::Decoder::new(u32_string_bytes);
    assert!(decoder.is_string());
    assert_eq!(decoder.read_string(), Some("ABC"));
    assert!(value_skipped(u32_string_bytes));
    assert_eq!(alisa::parse_abf(u32_string_bytes), Some(alisa::ABFValue::Str("ABC".into())));

}

#[test]
fn invalid_utf8() {

    let small_string_bytes = &[0b10000011, 128, 128, 128];
    let mut decoder = alisa::Decoder::new(small_string_bytes);
    assert!(decoder.is_string());
    assert_eq!(decoder.read_string(), None);
    assert!(value_skipped(small_string_bytes));
    assert_eq!(alisa::parse_abf(small_string_bytes), None);

}