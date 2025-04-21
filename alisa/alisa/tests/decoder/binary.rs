
use crate::value_skipped;

#[test]
fn binary_u8() {

    let u8_binary_bytes = &[0b11001111, 3, 65, 66, 67];
    let mut decoder = alisa::Decoder::new(u8_binary_bytes);
    assert!(decoder.is_binary());
    assert_eq!(decoder.read_binary(), Some(b"ABC".as_slice()));
    assert!(value_skipped(u8_binary_bytes));

    assert_eq!(alisa::parse_abf(u8_binary_bytes), Some(alisa::ABFNode::Binary(Box::new([65, 66, 67]))));

}

#[test]
fn binary_u16() {

    let u16_binary_bytes = &[0b11010000, 3, 0, 65, 66, 67];
    let mut decoder = alisa::Decoder::new(u16_binary_bytes);
    assert!(decoder.is_binary());
    assert_eq!(decoder.read_binary(), Some(b"ABC".as_slice()));
    assert!(value_skipped(u16_binary_bytes));

    assert_eq!(alisa::parse_abf(u16_binary_bytes), Some(alisa::ABFNode::Binary(Box::new([65, 66, 67]))));

}

#[test]
fn binary_u32() {

    let u32_binary_bytes = &[0b11010001, 3, 0, 0, 0, 65, 66, 67];
    let mut decoder = alisa::Decoder::new(u32_binary_bytes);
    assert!(decoder.is_binary());
    assert_eq!(decoder.read_binary(), Some(b"ABC".as_slice()));
    assert!(value_skipped(u32_binary_bytes));

    assert_eq!(alisa::parse_abf(u32_binary_bytes), Some(alisa::ABFNode::Binary(Box::new([65, 66, 67]))));

}
