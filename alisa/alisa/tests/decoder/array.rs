
use crate::value_skipped;

#[test]
fn small_array() {

    let small_array_bytes = &[0b10100011, 1, 2, 3];
    let mut decoder = alisa::Decoder::new(small_array_bytes);
    assert!(decoder.is_array());
    assert_eq!(decoder.read_array_length(), Some(3));
    assert!(decoder.is_u8());
    assert_eq!(decoder.read_u8(), Some(1));
    assert!(decoder.is_u8());
    assert_eq!(decoder.read_u8(), Some(2));
    assert!(decoder.is_u8());
    assert_eq!(decoder.read_u8(), Some(3));
    assert!(decoder.done());

    assert!(value_skipped(small_array_bytes));

}

#[test]
fn u8_array() {

    let u8_array_bytes = &[0b11010010, 3, 1, 2, 3];
    let mut decoder = alisa::Decoder::new(u8_array_bytes);
    assert!(decoder.is_array());
    assert_eq!(decoder.read_array_length(), Some(3));
    assert!(decoder.is_u8());
    assert_eq!(decoder.read_u8(), Some(1));
    assert!(decoder.is_u8());
    assert_eq!(decoder.read_u8(), Some(2));
    assert!(decoder.is_u8());
    assert_eq!(decoder.read_u8(), Some(3));
    assert!(decoder.done());

    assert!(value_skipped(u8_array_bytes));

    assert_eq!(alisa::parse_abf(u8_array_bytes), Some(alisa::ABFValue::Array(Box::new([
        alisa::ABFValue::PositiveInt(1),
        alisa::ABFValue::PositiveInt(2),
        alisa::ABFValue::PositiveInt(3),
    ]))));

}

#[test]
fn u16_array() {

    let u16_array_bytes = &[0b11010011, 3, 0, 1, 2, 3];
    let mut decoder = alisa::Decoder::new(u16_array_bytes);
    assert!(decoder.is_array());
    assert_eq!(decoder.read_array_length(), Some(3));
    assert!(decoder.is_u8());
    assert_eq!(decoder.read_u8(), Some(1));
    assert!(decoder.is_u8());
    assert_eq!(decoder.read_u8(), Some(2));
    assert!(decoder.is_u8());
    assert_eq!(decoder.read_u8(), Some(3));
    assert!(decoder.done());

    assert!(value_skipped(u16_array_bytes));

    assert_eq!(alisa::parse_abf(u16_array_bytes), Some(alisa::ABFValue::Array(Box::new([
        alisa::ABFValue::PositiveInt(1),
        alisa::ABFValue::PositiveInt(2),
        alisa::ABFValue::PositiveInt(3),
    ]))));

}

#[test]
fn u32_array() {

    let u32_array_bytes = &[0b11010100, 3, 0, 0, 0, 1, 2, 3];
    let mut decoder = alisa::Decoder::new(u32_array_bytes);
    assert!(decoder.is_array());
    assert_eq!(decoder.read_array_length(), Some(3));
    assert!(decoder.is_u8());
    assert_eq!(decoder.read_u8(), Some(1));
    assert!(decoder.is_u8());
    assert_eq!(decoder.read_u8(), Some(2));
    assert!(decoder.is_u8());
    assert_eq!(decoder.read_u8(), Some(3));
    assert!(decoder.done());

    assert!(value_skipped(u32_array_bytes));

    assert_eq!(alisa::parse_abf(u32_array_bytes), Some(alisa::ABFValue::Array(Box::new([
        alisa::ABFValue::PositiveInt(1),
        alisa::ABFValue::PositiveInt(2),
        alisa::ABFValue::PositiveInt(3),
    ]))));

}
