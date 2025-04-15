
use crate::encode;

#[test]
fn small_array() {

    assert_eq!(encode(|enc| {
        enc.array(0)
    }), &[0b10100000]);

    assert_eq!(encode(|enc| {
        enc.array(3)?;
        enc.u8(1)?;
        enc.u8(2)?;
        enc.u8(3)?;
        Ok(())
    }), &[0b10100011, 1, 2, 3]);

    assert_eq!(encode(|enc| {
        enc.array(15)
    }), &[0b10101111]);

}

#[test]
fn array_u8() {

    assert_eq!(encode(|enc| {
        enc.array(16)
    }), &[0b11010010, 16]);

    assert_eq!(encode(|enc| {
        enc.array(255)
    }), &[0b11010010, 255]);

}

#[test]
fn array_u16() {

    assert_eq!(encode(|enc| {
        enc.array(256)
    }), &[0b11010011, 0, 1]);

    assert_eq!(encode(|enc| {
        enc.array(u16::MAX as u32)
    }), &[0b11010011, 255, 255]);

}

#[test]
fn array_u32() {

    assert_eq!(encode(|enc| {
        enc.array(u16::MAX as u32 + 1)
    }), &[0b11010100, 0, 0, 1, 0]);

    assert_eq!(encode(|enc| {
        enc.array(u32::MAX)
    }), &[0b11010100, 255, 255, 255, 255]);

}
