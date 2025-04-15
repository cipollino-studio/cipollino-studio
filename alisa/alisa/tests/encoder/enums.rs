
use crate::encode;

#[test]
fn indexed_enums() {

    assert_eq!(encode(|enc| {
        enc.indexed_unit_enum(0)
    }), &[0b11011000]);

    assert_eq!(encode(|enc| {
        enc.indexed_unit_enum(7)
    }), &[0b11011111]);

    assert_eq!(encode(|enc| {
        enc.indexed_enum(0, |enc| {
            enc.u8(123)
        })
    }), &[0b11100000, 123]);

    assert_eq!(encode(|enc| {
        enc.indexed_enum(7, |enc| {
            enc.u8(123)
        })
    }), &[0b11100111, 123]);

}

#[test]
fn named_enums_small() {

    let long_name = String::from_utf8(vec![65; 127]).unwrap();

    assert_eq!(encode(|enc| {
        enc.named_unit_enum("ABC")
    }), &[0b11111110, 0b00000011, 65, 66, 67]);

    let mut bytes = vec![0b11111110, 0b01111111];
    for _ in 0..127 {
        bytes.push(65);
    }
    assert_eq!(encode(|enc| {
        enc.named_unit_enum(&long_name)
    }), bytes.as_slice());

    assert_eq!(encode(|enc| {
        enc.named_unit_enum("")
    }), &[0b11111110, 0b00000000]);

    assert_eq!(encode(|enc| {
        enc.named_enum("ABC", |enc| {
            enc.u8(123)
        })
    }), &[0b11111111, 0b00000011, 65, 66, 67, 123]);
    
    let mut bytes = vec![0b11111111, 0b01111111];
    for _ in 0..127 {
        bytes.push(65);
    }
    bytes.push(123);
    assert_eq!(encode(|enc| {
        enc.named_enum(&long_name, |enc| {
            enc.u8(123)
        })
    }), bytes.as_slice());

}

#[test]
fn named_enums_u8() {

    let long_name = String::from_utf8(vec![65; 128]).unwrap();

    let mut bytes = vec![0b11111110, 0b10000000, 128];
    for _ in 0..128 {
        bytes.push(65);
    }
    assert_eq!(encode(|enc| {
        enc.named_unit_enum(&long_name)
    }), bytes.as_slice());

    let mut bytes = vec![0b11111111, 0b10000000, 128];
    for _ in 0..128 {
        bytes.push(65);
    }
    bytes.push(123);
    assert_eq!(encode(|enc| {
        enc.named_enum(&long_name, |enc| {
            enc.u8(123)
        })
    }), bytes.as_slice());


    let long_name = String::from_utf8(vec![65; 255]).unwrap();

    let mut bytes = vec![0b11111110, 0b10000000, 255];
    for _ in 0..255 {
        bytes.push(65);
    }
    assert_eq!(encode(|enc| {
        enc.named_unit_enum(&long_name)
    }), bytes.as_slice());

    let mut bytes = vec![0b11111111, 0b10000000, 255];
    for _ in 0..255 {
        bytes.push(65);
    }
    bytes.push(123);
    assert_eq!(encode(|enc| {
        enc.named_enum(&long_name, |enc| {
            enc.u8(123)
        })
    }), bytes.as_slice());

}

#[test]
fn named_enums_u16() {

    let long_name = String::from_utf8(vec![65; 256]).unwrap();

    let mut bytes = vec![0b11111110, 0b10000001, 0, 1];
    for _ in 0..256 {
        bytes.push(65);
    }
    assert_eq!(encode(|enc| {
        enc.named_unit_enum(&long_name)
    }), bytes.as_slice());

    let mut bytes = vec![0b11111111, 0b10000001, 0, 1];
    for _ in 0..256 {
        bytes.push(65);
    }
    bytes.push(123);
    assert_eq!(encode(|enc| {
        enc.named_enum(&long_name, |enc| {
            enc.u8(123)
        })
    }), bytes.as_slice());


    let long_name = String::from_utf8(vec![65; u16::MAX as usize]).unwrap();

    let mut bytes = vec![0b11111110, 0b10000001, 255, 255];
    for _ in 0..u16::MAX {
        bytes.push(65);
    }
    assert_eq!(encode(|enc| {
        enc.named_unit_enum(&long_name)
    }), bytes.as_slice());

    let mut bytes = vec![0b11111111, 0b10000001, 255, 255];
    for _ in 0..u16::MAX {
        bytes.push(65);
    }
    bytes.push(123);
    assert_eq!(encode(|enc| {
        enc.named_enum(&long_name, |enc| {
            enc.u8(123)
        })
    }), bytes.as_slice());

}

#[test]
fn named_enums_u32() {

    let long_name = String::from_utf8(vec![65; u16::MAX as usize + 1]).unwrap();

    let mut bytes = vec![0b11111110, 0b10000010, 0, 0, 1, 0];
    for _ in 0..(u16::MAX as usize + 1) {
        bytes.push(65);
    }
    assert_eq!(encode(|enc| {
        enc.named_unit_enum(&long_name)
    }), bytes.as_slice());

    let mut bytes = vec![0b11111111, 0b10000010, 0, 0, 1, 0];
    for _ in 0..(u16::MAX as usize + 1) {
        bytes.push(65);
    }
    bytes.push(123);
    assert_eq!(encode(|enc| {
        enc.named_enum(&long_name, |enc| {
            enc.u8(123)
        })
    }), bytes.as_slice());

}
