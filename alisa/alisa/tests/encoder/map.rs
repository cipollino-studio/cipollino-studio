
use crate::encode;

#[test]
fn map_small() {

    assert_eq!(encode(|enc| {
        enc.map(0)
    }), &[0b10110000]);

    assert_eq!(encode(|enc| {
        enc.map(3)?;
        enc.map_field("A", |enc| {
            enc.u8(1)
        })?;
        enc.map_field("B", |enc| {
            enc.u8(2)
        })?;
        enc.map_field("C", |enc| {
            enc.u8(3)
        })?;
        Ok(())
    }), &[0b10110011, 1, 65, 1, 1, 66, 2, 1, 67, 3]);

    assert_eq!(encode(|enc| {
        enc.map(15)
    }), &[0b10111111]);

}

#[test]
fn map_u8() {

    assert_eq!(encode(|enc| {
        enc.map(16)
    }), &[0b11010101, 16]);

    assert_eq!(encode(|enc| {
        enc.map(255)
    }), &[0b11010101, 255]);

}

#[test]
fn map_u16() {

    assert_eq!(encode(|enc| {
        enc.map(256)
    }), &[0b11010110, 0, 1]);

    assert_eq!(encode(|enc| {
        enc.map(u16::MAX as u32)
    }), &[0b11010110, 255, 255]);

}

#[test]
fn map_u32() {

    assert_eq!(encode(|enc| {
        enc.map(u16::MAX as u32 + 1)
    }), &[0b11010111, 0, 0, 1, 0]);

}
