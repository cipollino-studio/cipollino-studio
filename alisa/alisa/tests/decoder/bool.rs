
use crate::value_skipped;

#[test]
fn bool() {

    let false_bytes = &[0b11000000];
    let mut decoder = alisa::Decoder::new(false_bytes);
    assert!(decoder.is_bool());
    assert_eq!(decoder.read_bool(), Some(false));
    assert!(value_skipped(false_bytes));
    assert_eq!(alisa::parse_abf(false_bytes), Some(alisa::ABFValue::Bool(false)));

    let true_bytes = &[0b11000001];
    let mut decoder = alisa::Decoder::new(true_bytes);
    assert!(decoder.is_bool());
    assert_eq!(decoder.read_bool(), Some(true));
    assert!(value_skipped(true_bytes));
    assert_eq!(alisa::parse_abf(true_bytes), Some(alisa::ABFValue::Bool(true)));

}
