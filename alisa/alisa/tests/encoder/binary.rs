
use crate::encode;

#[test]
fn binary() {

    assert_eq!(encode(|enc| {
        enc.binary(&[])
    }), &[0b11001111, 0]);

    assert_eq!(encode(|enc| {
        enc.binary("ABC".as_bytes())
    }), &[0b11001111, 3, 65, 66, 67]);

    let mut binary_bytes = vec![0b11001111, 255];
    for _ in 0..255 {
        binary_bytes.push(65);
    }
    assert_eq!(encode(|enc| {
        enc.binary(&[65; 255])
    }), binary_bytes.as_slice());

    let mut binary_bytes = vec![0b11010000, 0, 1];
    for _ in 0..256 {
        binary_bytes.push(65);
    }
    assert_eq!(encode(|enc| {
        enc.binary(&[65; 256])
    }), binary_bytes.as_slice());

    let mut binary_bytes = vec![0b11010000, 255, 255];
    for _ in 0..u16::MAX {
        binary_bytes.push(65);
    }
    assert_eq!(encode(|enc| {
        enc.binary(&[65; u16::MAX as usize])
    }), binary_bytes.as_slice());

    let mut binary_bytes = vec![0b11010001, 0, 0, 1, 0];
    for _ in 0..(u16::MAX as usize + 1) {
        binary_bytes.push(65);
    }
    assert_eq!(encode(|enc| {
        enc.binary(&[65; u16::MAX as usize + 1])
    }), binary_bytes.as_slice());

}
