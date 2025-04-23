use crate::value_skipped;


#[test]
fn uint_small() {

    let positive_int_bytes = &[56];
    let mut decoder = alisa::Decoder::new(positive_int_bytes);
    assert!(decoder.is_u8());
    assert!(decoder.is_u16());
    assert!(decoder.is_u32());
    assert!(decoder.is_u64());

    let mut decoder = alisa::Decoder::new(positive_int_bytes);
    assert_eq!(decoder.read_u8(), Some(56));
    let mut decoder = alisa::Decoder::new(positive_int_bytes);
    assert_eq!(decoder.read_u16(), Some(56));
    let mut decoder = alisa::Decoder::new(positive_int_bytes);
    assert_eq!(decoder.read_u32(), Some(56));
    let mut decoder = alisa::Decoder::new(positive_int_bytes);
    assert_eq!(decoder.read_u64(), Some(56));

    assert!(value_skipped(positive_int_bytes));

    assert_eq!(alisa::parse_abf(positive_int_bytes), Some(alisa::ABFValue::PositiveInt(56)));

}

#[test]
fn uint_u8() {

    let u8_bytes = &[0b11000010, 254];
    let mut decoder = alisa::Decoder::new(u8_bytes);
    assert!(decoder.is_u8());
    assert!(decoder.is_u16());
    assert!(decoder.is_u32());
    assert!(decoder.is_u64());
    
    let mut decoder = alisa::Decoder::new(u8_bytes);
    assert_eq!(decoder.read_u8(), Some(254));
    let mut decoder = alisa::Decoder::new(u8_bytes);
    assert_eq!(decoder.read_u16(), Some(254));
    let mut decoder = alisa::Decoder::new(u8_bytes);
    assert_eq!(decoder.read_u32(), Some(254));
    let mut decoder = alisa::Decoder::new(u8_bytes);
    assert_eq!(decoder.read_u64(), Some(254));

    assert!(value_skipped(u8_bytes));

    assert_eq!(alisa::parse_abf(u8_bytes), Some(alisa::ABFValue::U8(254)));

}

#[test]
fn uint_u16() {

    let u16_bytes = &[0b11000011, 3, 1];
    let mut decoder = alisa::Decoder::new(u16_bytes);
    assert!(!decoder.is_u8());
    assert!(decoder.is_u16());
    assert!(decoder.is_u32());
    assert!(decoder.is_u64());
    
    let mut decoder = alisa::Decoder::new(u16_bytes);
    assert_eq!(decoder.read_u8(), None);
    assert_eq!(decoder.read_u16(), Some(259));
    let mut decoder = alisa::Decoder::new(u16_bytes);
    assert_eq!(decoder.read_u32(), Some(259));
    let mut decoder = alisa::Decoder::new(u16_bytes);
    assert_eq!(decoder.read_u64(), Some(259));

    assert!(value_skipped(u16_bytes));

    assert_eq!(alisa::parse_abf(u16_bytes), Some(alisa::ABFValue::U16(259)));

}

#[test]
fn uint_u32() {

    let u32_bytes = &[0b11000100, 1, 1, 1, 0];
    let mut decoder = alisa::Decoder::new(u32_bytes);
    assert!(!decoder.is_u8());
    assert!(!decoder.is_u16());
    assert!(decoder.is_u32());
    assert!(decoder.is_u64());
    
    let mut decoder = alisa::Decoder::new(u32_bytes);
    assert_eq!(decoder.read_u8(), None);
    assert_eq!(decoder.read_u16(), None);
    assert_eq!(decoder.read_u32(), Some(65793));
    let mut decoder = alisa::Decoder::new(u32_bytes);
    assert_eq!(decoder.read_u64(), Some(65793));

    assert!(value_skipped(u32_bytes));

    assert_eq!(alisa::parse_abf(u32_bytes), Some(alisa::ABFValue::U32(65793)));

}

#[test]
fn uint_u64() {

    let u64_bytes = &[0b11000101, 1, 1, 1, 0, 1, 0, 0, 0];
    let mut decoder = alisa::Decoder::new(u64_bytes);
    assert!(!decoder.is_u8());
    assert!(!decoder.is_u16());
    assert!(!decoder.is_u32());
    assert!(decoder.is_u64());
    
    let mut decoder = alisa::Decoder::new(u64_bytes);
    assert_eq!(decoder.read_u8(), None);
    assert_eq!(decoder.read_u16(), None);
    assert_eq!(decoder.read_u32(), None);
    assert_eq!(decoder.read_u64(), Some(4295033089));

    assert!(value_skipped(u64_bytes));

    assert_eq!(alisa::parse_abf(u64_bytes), Some(alisa::ABFValue::U64(4295033089)));

}
