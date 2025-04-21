
use crate::value_skipped;

#[test]
fn int_small() {

    let positive_int_bytes = &[56];
    let mut decoder = alisa::Decoder::new(positive_int_bytes);
    assert!(decoder.is_i8());
    assert!(decoder.is_i16());
    assert!(decoder.is_i32());
    assert!(decoder.is_i64());

    let mut decoder = alisa::Decoder::new(positive_int_bytes);
    assert_eq!(decoder.read_i8(), Some(56));
    let mut decoder = alisa::Decoder::new(positive_int_bytes);
    assert_eq!(decoder.read_i16(), Some(56));
    let mut decoder = alisa::Decoder::new(positive_int_bytes);
    assert_eq!(decoder.read_i32(), Some(56));
    let mut decoder = alisa::Decoder::new(positive_int_bytes);
    assert_eq!(decoder.read_i64(), Some(56));

    assert!(value_skipped(positive_int_bytes));

    assert_eq!(alisa::parse_abf(positive_int_bytes), Some(alisa::ABFNode::PositiveInt(56)));

}

#[test]
fn int_i8() {

    let i8_bytes = &[0b11000110, 254];
    let mut decoder = alisa::Decoder::new(i8_bytes);
    assert!(decoder.is_i8());
    assert!(decoder.is_i16());
    assert!(decoder.is_i32());
    assert!(decoder.is_i64());
    
    let mut decoder = alisa::Decoder::new(i8_bytes);
    assert_eq!(decoder.read_i8(), Some(-2));
    let mut decoder = alisa::Decoder::new(i8_bytes);
    assert_eq!(decoder.read_i16(), Some(-2));
    let mut decoder = alisa::Decoder::new(i8_bytes);
    assert_eq!(decoder.read_i32(), Some(-2));
    let mut decoder = alisa::Decoder::new(i8_bytes);
    assert_eq!(decoder.read_i64(), Some(-2));

    assert!(value_skipped(i8_bytes));
    assert_eq!(alisa::parse_abf(i8_bytes), Some(alisa::ABFNode::I8(-2)));

}

#[test]
fn int_i16() {

    let i16_bytes = &[0b11000111, 3, 1];
    let mut decoder = alisa::Decoder::new(i16_bytes);
    assert!(!decoder.is_i8());
    assert!(decoder.is_i16());
    assert!(decoder.is_i32());
    assert!(decoder.is_i64());
    
    let mut decoder = alisa::Decoder::new(i16_bytes);
    assert_eq!(decoder.read_i8(), None);
    assert_eq!(decoder.read_i16(), Some(259));
    let mut decoder = alisa::Decoder::new(i16_bytes);
    assert_eq!(decoder.read_i32(), Some(259));
    let mut decoder = alisa::Decoder::new(i16_bytes);
    assert_eq!(decoder.read_i64(), Some(259));

    assert!(value_skipped(i16_bytes));
    assert_eq!(alisa::parse_abf(i16_bytes), Some(alisa::ABFNode::I16(259)));

}

#[test]
fn int_i32() {

    let i32_bytes = &[0b11001000, 1, 1, 1, 0];
    let mut decoder = alisa::Decoder::new(i32_bytes);
    assert!(!decoder.is_i8());
    assert!(!decoder.is_i16());
    assert!(decoder.is_i32());
    assert!(decoder.is_i64());
    
    let mut decoder = alisa::Decoder::new(i32_bytes);
    assert_eq!(decoder.read_i8(), None);
    assert_eq!(decoder.read_i16(), None);
    assert_eq!(decoder.read_i32(), Some(65793));
    let mut decoder = alisa::Decoder::new(i32_bytes);
    assert_eq!(decoder.read_i64(), Some(65793));

    assert!(value_skipped(i32_bytes));
    assert_eq!(alisa::parse_abf(i32_bytes), Some(alisa::ABFNode::I32(65793)));

}

#[test]
fn int_i64() {

    let i64_bytes = &[0b11001001, 1, 1, 1, 0, 1, 0, 0, 0];
    let mut decoder = alisa::Decoder::new(i64_bytes);
    assert!(!decoder.is_i8());
    assert!(!decoder.is_i16());
    assert!(!decoder.is_i32());
    assert!(decoder.is_i64());
    
    let mut decoder = alisa::Decoder::new(i64_bytes);
    assert_eq!(decoder.read_i8(), None);
    assert_eq!(decoder.read_i16(), None);
    assert_eq!(decoder.read_i32(), None);
    assert_eq!(decoder.read_i64(), Some(4295033089));

    assert!(value_skipped(i64_bytes));
    assert_eq!(alisa::parse_abf(i64_bytes), Some(alisa::ABFNode::I64(4295033089)));

}
