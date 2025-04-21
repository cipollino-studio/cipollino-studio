
use crate::value_skipped;

#[test]
fn small_obj_ptr_small_type() {

    let bytes = &[0b11101010, 0, 1, 0, 0];
    let mut decoder = alisa::Decoder::new(bytes);
    assert!(decoder.is_obj_ptr());
    assert_eq!(decoder.read_obj_ptr(), Some((2, 256)));
    assert!(value_skipped(bytes));
    assert_eq!(alisa::parse_abf(bytes), Some(alisa::ABFNode::ObjPtr(2, 256)));

}

#[test]
fn large_obj_ptr_small_type() {

    let bytes = &[0b11110010, 0, 1, 0, 0, 0, 0, 0, 0];
    let mut decoder = alisa::Decoder::new(bytes);
    assert!(decoder.is_obj_ptr());
    assert_eq!(decoder.read_obj_ptr(), Some((2, 256)));
    assert!(value_skipped(bytes));
    assert_eq!(alisa::parse_abf(bytes), Some(alisa::ABFNode::ObjPtr(2, 256)));

}

#[test]
fn small_obj_ptr_u8_type() {

    let bytes = &[0b11111001, 16, 0, 1, 0, 0];
    let mut decoder = alisa::Decoder::new(bytes);
    assert!(decoder.is_obj_ptr());
    assert_eq!(decoder.read_obj_ptr(), Some((16, 256)));
    assert!(value_skipped(bytes));
    assert_eq!(alisa::parse_abf(bytes), Some(alisa::ABFNode::ObjPtr(16, 256)));

}

#[test]
fn large_obj_ptr_u8_type() {

    let bytes = &[0b11111010, 16, 0, 1, 0, 0, 0, 0, 0, 0];
    let mut decoder = alisa::Decoder::new(bytes);
    assert!(decoder.is_obj_ptr());
    assert_eq!(decoder.read_obj_ptr(), Some((16, 256)));
    assert!(value_skipped(bytes));
    assert_eq!(alisa::parse_abf(bytes), Some(alisa::ABFNode::ObjPtr(16, 256)));

}

#[test]
fn small_obj_ptr_u16_type() {

    let bytes = &[0b11111011, 16, 1, 0, 1, 0, 0];
    let mut decoder = alisa::Decoder::new(bytes);
    assert!(decoder.is_obj_ptr());
    assert_eq!(decoder.read_obj_ptr(), Some((272, 256)));
    assert!(value_skipped(bytes));
    assert_eq!(alisa::parse_abf(bytes), Some(alisa::ABFNode::ObjPtr(272, 256)));

}

#[test]
fn large_obj_ptr_u16_type() {

    let bytes = &[0b11111100, 16, 1, 0, 1, 0, 0, 0, 0, 0, 0];
    let mut decoder = alisa::Decoder::new(bytes);
    assert!(decoder.is_obj_ptr());
    assert_eq!(decoder.read_obj_ptr(), Some((272, 256)));
    assert!(value_skipped(bytes));
    assert_eq!(alisa::parse_abf(bytes), Some(alisa::ABFNode::ObjPtr(272, 256)));

}
