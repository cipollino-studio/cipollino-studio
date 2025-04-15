
use crate::value_skipped;

#[test]
fn small_map() {

    let bytes = &[
        0b10110100,
        0b00000011, 65, 66, 67, 1,
        0b10000000, 2, 65, 66, 2,
        0b10000001, 2, 0, 67, 68, 3,
        0b10000010, 2, 0, 0, 0, 69, 70, 4
    ];
    let mut decoder = alisa::Decoder::new(bytes);
    assert!(decoder.is_map());
    assert_eq!(decoder.read_map_length(), Some(4));
    assert_eq!(decoder.read_map_field_name(), Some("ABC"));
    assert_eq!(decoder.read_u8(), Some(1));
    assert_eq!(decoder.read_map_field_name(), Some("AB"));
    assert_eq!(decoder.read_u8(), Some(2));
    assert_eq!(decoder.read_map_field_name(), Some("CD"));
    assert_eq!(decoder.read_u8(), Some(3));
    assert_eq!(decoder.read_map_field_name(), Some("EF"));
    assert_eq!(decoder.read_u8(), Some(4));
    assert!(decoder.done());
    assert!(value_skipped(bytes));

}

#[test]
fn u8_map() {

    let bytes = &[
        0b11010101, 4,
        0b00000011, 65, 66, 67, 1,
        0b10000000, 2, 65, 66, 2,
        0b10000001, 2, 0, 67, 68, 3,
        0b10000010, 2, 0, 0, 0, 69, 70, 4
    ];
    let mut decoder = alisa::Decoder::new(bytes);
    assert!(decoder.is_map());
    assert_eq!(decoder.read_map_length(), Some(4));
    assert_eq!(decoder.read_map_field_name(), Some("ABC"));
    assert_eq!(decoder.read_u8(), Some(1));
    assert_eq!(decoder.read_map_field_name(), Some("AB"));
    assert_eq!(decoder.read_u8(), Some(2));
    assert_eq!(decoder.read_map_field_name(), Some("CD"));
    assert_eq!(decoder.read_u8(), Some(3));
    assert_eq!(decoder.read_map_field_name(), Some("EF"));
    assert_eq!(decoder.read_u8(), Some(4));
    assert!(decoder.done());
    assert!(value_skipped(bytes));

}

#[test]
fn u16_map() {

    let bytes = &[
        0b11010110, 4, 0,
        0b00000011, 65, 66, 67, 1,
        0b10000000, 2, 65, 66, 2,
        0b10000001, 2, 0, 67, 68, 3,
        0b10000010, 2, 0, 0, 0, 69, 70, 4
    ];
    let mut decoder = alisa::Decoder::new(bytes);
    assert!(decoder.is_map());
    assert_eq!(decoder.read_map_length(), Some(4));
    assert_eq!(decoder.read_map_field_name(), Some("ABC"));
    assert_eq!(decoder.read_u8(), Some(1));
    assert_eq!(decoder.read_map_field_name(), Some("AB"));
    assert_eq!(decoder.read_u8(), Some(2));
    assert_eq!(decoder.read_map_field_name(), Some("CD"));
    assert_eq!(decoder.read_u8(), Some(3));
    assert_eq!(decoder.read_map_field_name(), Some("EF"));
    assert_eq!(decoder.read_u8(), Some(4));
    assert!(decoder.done());
    assert!(value_skipped(bytes));

}

#[test]
fn u32_map() {

    let bytes = &[
        0b11010111, 4, 0, 0, 0,
        0b00000011, 65, 66, 67, 1,
        0b10000000, 2, 65, 66, 2,
        0b10000001, 2, 0, 67, 68, 3,
        0b10000010, 2, 0, 0, 0, 69, 70, 4
    ];
    let mut decoder = alisa::Decoder::new(bytes);
    assert!(decoder.is_map());
    assert_eq!(decoder.read_map_length(), Some(4));
    assert_eq!(decoder.read_map_field_name(), Some("ABC"));
    assert_eq!(decoder.read_u8(), Some(1));
    assert_eq!(decoder.read_map_field_name(), Some("AB"));
    assert_eq!(decoder.read_u8(), Some(2));
    assert_eq!(decoder.read_map_field_name(), Some("CD"));
    assert_eq!(decoder.read_u8(), Some(3));
    assert_eq!(decoder.read_map_field_name(), Some("EF"));
    assert_eq!(decoder.read_u8(), Some(4));
    assert!(decoder.done());
    assert!(value_skipped(bytes));

}
