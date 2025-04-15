
use crate::encode;

#[test]
fn small_string() {

    assert_eq!(encode(|enc| {
        enc.string("")
    }), &[0b10000000]);

    assert_eq!(encode(|enc| {
        enc.string("ABC")
    }), &[0b10000011, 65, 66, 67]);

    assert_eq!(encode(|enc| {
        enc.string("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA")
    }), &[0b10011111, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65]);

    assert_eq!(encode(|enc| {
        enc.string("\0")
    }), &[0b10000001, 0]);

}

#[test]
fn string_u8() {

    let mut str_bytes = vec![0b11001100, 32];
    for _ in 0..32 {
        str_bytes.push(65);
    }
    assert_eq!(encode(|enc| {
        enc.string("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA")
    }), str_bytes.as_slice());

    let string = String::from_utf8([65; 255].to_vec()).unwrap();
    let mut str_bytes = vec![0b11001100, 255];
    for _ in 0..255 {
        str_bytes.push(65);
    }
    assert_eq!(encode(|enc| {
        enc.string(&string)
    }), str_bytes.as_slice());

}

#[test]
fn string_u16() {

    let string = String::from_utf8([65; 256].to_vec()).unwrap();
    let mut str_bytes = vec![0b11001101, 0, 1];
    for _ in 0..256 {
        str_bytes.push(65);
    }
    assert_eq!(encode(|enc| {
        enc.string(&string)
    }), str_bytes.as_slice());

    let string = String::from_utf8([65; u16::MAX as usize].to_vec()).unwrap();
    let mut str_bytes = vec![0b11001101, 255, 255];
    for _ in 0..u16::MAX {
        str_bytes.push(65);
    }
    assert_eq!(encode(|enc| {
        enc.string(&string)
    }), str_bytes.as_slice());

}

#[test]
fn string_u32() {

    let string = String::from_utf8([65; u16::MAX as usize + 1].to_vec()).unwrap();
    let mut str_bytes = vec![0b11001110, 0, 0, 1, 0];
    for _ in 0..(u16::MAX as usize + 1) {
        str_bytes.push(65);
    }
    assert_eq!(encode(|enc| {
        enc.string(&string)
    }), str_bytes.as_slice());

}
