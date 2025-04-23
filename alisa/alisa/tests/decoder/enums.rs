
use crate::value_skipped;

#[test]
fn indexed_unit_enum() {

    let bytes = &[0b11011010];
    let mut decoder = alisa::Decoder::new(bytes);
    assert!(decoder.is_indexed_unit_enum());
    assert_eq!(decoder.read_indexed_unit_enum(), Some(2));
    assert!(value_skipped(bytes));
    assert_eq!(alisa::parse_abf(bytes), Some(alisa::ABFValue::IndexedUnitEnum(2)));

}

#[test]
fn indexed_enum() {

    let bytes = &[0b11100010, 5];
    let mut decoder = alisa::Decoder::new(bytes);
    assert!(decoder.is_indexed_enum());
    assert_eq!(decoder.read_indexed_enum(), Some(2));
    assert!(decoder.is_u8());
    assert_eq!(decoder.read_u8(), Some(5));
    assert!(value_skipped(bytes));
    assert_eq!(alisa::parse_abf(bytes), Some(alisa::ABFValue::IndexedEnum(2, Box::new(alisa::ABFValue::PositiveInt(5)))));

}

#[test]
fn named_unit_enum_small_name() {

    let bytes = &[0b11111110, 0b00000011, 65, 66, 67];
    let mut decoder = alisa::Decoder::new(bytes);
    assert!(decoder.is_named_unit_enum());
    assert_eq!(decoder.read_named_unit_enum(), Some("ABC"));
    assert!(value_skipped(bytes));
    assert_eq!(alisa::parse_abf(bytes), Some(alisa::ABFValue::NamedUnitEnum("ABC".into())));

}

#[test]
fn named_unit_enum_u8_name() {

    let bytes = &[0b11111110, 0b10000000, 3, 65, 66, 67];
    let mut decoder = alisa::Decoder::new(bytes);
    assert!(decoder.is_named_unit_enum());
    assert_eq!(decoder.read_named_unit_enum(), Some("ABC"));
    assert!(value_skipped(bytes));
    assert_eq!(alisa::parse_abf(bytes), Some(alisa::ABFValue::NamedUnitEnum("ABC".into())));

}

#[test]
fn named_unit_enum_u16_name() {

    let bytes = &[0b11111110, 0b10000001, 3, 0, 65, 66, 67];
    let mut decoder = alisa::Decoder::new(bytes);
    assert!(decoder.is_named_unit_enum());
    assert_eq!(decoder.read_named_unit_enum(), Some("ABC"));
    assert!(value_skipped(bytes));
    assert_eq!(alisa::parse_abf(bytes), Some(alisa::ABFValue::NamedUnitEnum("ABC".into())));

}

#[test]
fn named_unit_enum_u32_name() {

    let bytes = &[0b11111110, 0b10000010, 3, 0, 0, 0, 65, 66, 67];
    let mut decoder = alisa::Decoder::new(bytes);
    assert!(decoder.is_named_unit_enum());
    assert_eq!(decoder.read_named_unit_enum(), Some("ABC"));
    assert!(value_skipped(bytes));

}

#[test]
fn named_enum_small_name() {

    let bytes = &[0b11111111, 0b00000011, 65, 66, 67, 5];
    let mut decoder = alisa::Decoder::new(bytes);
    assert!(decoder.is_named_enum());
    assert_eq!(decoder.read_named_enum(), Some("ABC"));
    assert!(decoder.is_u8());
    assert_eq!(decoder.read_u8(), Some(5));
    assert!(value_skipped(bytes));
    assert_eq!(alisa::parse_abf(bytes), Some(alisa::ABFValue::NamedEnum("ABC".into(), Box::new(alisa::ABFValue::PositiveInt(5)))));

}

#[test]
fn named_enum_u8_name() {

    let bytes = &[0b11111111, 0b10000000, 3, 65, 66, 67, 5];
    let mut decoder = alisa::Decoder::new(bytes);
    assert!(decoder.is_named_enum());
    assert_eq!(decoder.read_named_enum(), Some("ABC"));
    assert!(decoder.is_u8());
    assert_eq!(decoder.read_u8(), Some(5));
    assert!(value_skipped(bytes));
    assert_eq!(alisa::parse_abf(bytes), Some(alisa::ABFValue::NamedEnum("ABC".into(), Box::new(alisa::ABFValue::PositiveInt(5)))));

}

#[test]
fn named_enum_u16_name() {

    let bytes = &[0b11111111, 0b10000001, 3, 0, 65, 66, 67, 5];
    let mut decoder = alisa::Decoder::new(bytes);
    assert!(decoder.is_named_enum());
    assert_eq!(decoder.read_named_enum(), Some("ABC"));
    assert!(decoder.is_u8());
    assert_eq!(decoder.read_u8(), Some(5));
    assert!(value_skipped(bytes));
    assert_eq!(alisa::parse_abf(bytes), Some(alisa::ABFValue::NamedEnum("ABC".into(), Box::new(alisa::ABFValue::PositiveInt(5)))));

}

#[test]
fn named_enum_u32_name() {

    let bytes = &[0b11111111, 0b10000010, 3, 0, 0, 0, 65, 66, 67, 5];
    let mut decoder = alisa::Decoder::new(bytes);
    assert!(decoder.is_named_enum());
    assert_eq!(decoder.read_named_enum(), Some("ABC"));
    assert!(decoder.is_u8());
    assert_eq!(decoder.read_u8(), Some(5));
    assert!(value_skipped(bytes));
    assert_eq!(alisa::parse_abf(bytes), Some(alisa::ABFValue::NamedEnum("ABC".into(), Box::new(alisa::ABFValue::PositiveInt(5)))));


}
