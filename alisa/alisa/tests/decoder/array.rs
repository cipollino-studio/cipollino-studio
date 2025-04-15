
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

}

#[test]
fn u16_array() {

    let u8_array_bytes = &[0b11010011, 3, 0, 1, 2, 3];
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

}

#[test]
fn u32_array() {

    let u8_array_bytes = &[0b11010100, 3, 0, 0, 0, 1, 2, 3];
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

}
