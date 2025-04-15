
use crate::encode;

#[test]
fn int8() {

    assert_eq!(encode(|enc| {
        enc.i8(5)
    }), &[5]);

    assert_eq!(encode(|enc| {
        enc.i8(127)
    }), &[127]);

    assert_eq!(encode(|enc| {
        enc.i8(-5)
    }), &[0b11000110, 0b11111011]);

    assert_eq!(encode(|enc| {
        enc.i8(-128)
    }), &[0b11000110, 0b10000000]);

}

#[test]
fn int16() {

    assert_eq!(encode(|enc| {
        enc.i16(5)
    }), &[5]);

    assert_eq!(encode(|enc| {
        enc.i16(127)
    }), &[127]);

    assert_eq!(encode(|enc| {
        enc.i16(128)
    }), &[0b11000111, 128, 0]);

    assert_eq!(encode(|enc| {
        enc.i16(256)
    }), &[0b11000111, 0, 1]);

    assert_eq!(encode(|enc| {
        enc.i16(i16::MAX)
    }), &[0b11000111, 255, 127]);

    assert_eq!(encode(|enc| {
        enc.i16(-5)
    }), &[0b11000110, 0b11111011]);

    assert_eq!(encode(|enc| {
        enc.i16(-128)
    }), &[0b11000110, 0b10000000]);

    assert_eq!(encode(|enc| {
        enc.i16(-129)
    }), &[0b11000111, 127, 255]);

    assert_eq!(encode(|enc| {
        enc.i16(i16::MIN)
    }), &[0b11000111, 0, 128]);
    
}

#[test]
fn int32() {

    assert_eq!(encode(|enc| {
        enc.i32(5)
    }), &[5]);

    assert_eq!(encode(|enc| {
        enc.i32(127)
    }), &[127]);

    assert_eq!(encode(|enc| {
        enc.i32(128)
    }), &[0b11000111, 128, 0]);

    assert_eq!(encode(|enc| {
        enc.i32(256)
    }), &[0b11000111, 0, 1]);

    assert_eq!(encode(|enc| {
        enc.i32(i16::MAX as i32)
    }), &[0b11000111, 255, 127]);

    assert_eq!(encode(|enc| {
        enc.i32(i16::MAX as i32 + 1)
    }), &[0b11001000, 0, 128, 0, 0]);

    assert_eq!(encode(|enc| {
        enc.i32(i32::MAX as i32)
    }), &[0b11001000, 255, 255, 255, 127]);

    assert_eq!(encode(|enc| {
        enc.i32(-5)
    }), &[0b11000110, 0b11111011]);

    assert_eq!(encode(|enc| {
        enc.i32(-128)
    }), &[0b11000110, 0b10000000]);

    assert_eq!(encode(|enc| {
        enc.i32(-129)
    }), &[0b11000111, 127, 255]);

    assert_eq!(encode(|enc| {
        enc.i32(i16::MIN as i32)
    }), &[0b11000111, 0, 128]);

    assert_eq!(encode(|enc| {
        enc.i32(i16::MIN as i32 - 1)
    }), &[0b11001000, 255, 127, 255, 255]);

    assert_eq!(encode(|enc| {
        enc.i32(i32::MIN)
    }), &[0b11001000, 0, 0, 0, 128]);

}

#[test]
fn uint64() {

    assert_eq!(encode(|enc| {
        enc.i64(5)
    }), &[5]);

    assert_eq!(encode(|enc| {
        enc.i64(127)
    }), &[127]);

    assert_eq!(encode(|enc| {
        enc.i64(128)
    }), &[0b11000111, 128, 0]);

    assert_eq!(encode(|enc| {
        enc.i64(256)
    }), &[0b11000111, 0, 1]);

    assert_eq!(encode(|enc| {
        enc.i64(i16::MAX as i64)
    }), &[0b11000111, 255, 127]);

    assert_eq!(encode(|enc| {
        enc.i64(i16::MAX as i64 + 1)
    }), &[0b11001000, 0, 128, 0, 0]);

    assert_eq!(encode(|enc| {
        enc.i64(i32::MAX as i64)
    }), &[0b11001000, 255, 255, 255, 127]);

    assert_eq!(encode(|enc| {
        enc.i64(i32::MAX as i64 + 1)
    }), &[0b11001001, 0, 0, 0, 128, 0, 0, 0, 0]);

    assert_eq!(encode(|enc| {
        enc.i64(i64::MAX)
    }), &[0b11001001, 255, 255, 255, 255, 255, 255, 255, 127]);

    assert_eq!(encode(|enc| {
        enc.i64(-5)
    }), &[0b11000110, 0b11111011]);

    assert_eq!(encode(|enc| {
        enc.i64(-128)
    }), &[0b11000110, 0b10000000]);

    assert_eq!(encode(|enc| {
        enc.i64(-129)
    }), &[0b11000111, 127, 255]);

    assert_eq!(encode(|enc| {
        enc.i64(i16::MIN as i64)
    }), &[0b11000111, 0, 128]);

    assert_eq!(encode(|enc| {
        enc.i64(i16::MIN as i64 - 1)
    }), &[0b11001000, 255, 127, 255, 255]);

    assert_eq!(encode(|enc| {
        enc.i64(i32::MIN as i64)
    }), &[0b11001000, 0, 0, 0, 128]);

    assert_eq!(encode(|enc| {
        enc.i64(i32::MIN as i64 - 1)
    }), &[0b11001001, 255, 255, 255, 127, 255, 255, 255, 255]);

    assert_eq!(encode(|enc| {
        enc.i64(i64::MIN)
    }), &[0b11001001, 0, 0, 0, 0, 0, 0, 0, 128]);

}
