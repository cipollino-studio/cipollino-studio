
use crate::value_skipped;

#[test]
fn f32() {

    let f32_bytes = &[0b11001010, 0, 0, 0, 0];
    let mut decoder = alisa::Decoder::new(f32_bytes);
    assert!(decoder.is_f32());
    assert_eq!(decoder.read_f32(), Some(0.0));
    assert!(value_skipped(f32_bytes));
    assert_eq!(alisa::parse_abf(f32_bytes), Some(alisa::ABFValue::F32(0.0)));

}

#[test]
fn f64() {

    let f64_bytes = &[0b11001011, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut decoder = alisa::Decoder::new(f64_bytes);
    assert!(decoder.is_f64());
    assert_eq!(decoder.read_f64(), Some(0.0));
    assert!(value_skipped(f64_bytes));
    assert_eq!(alisa::parse_abf(f64_bytes), Some(alisa::ABFValue::F64(0.0)));

}
