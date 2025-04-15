
use crate::encode;

#[test]
fn obj_ptr() {
    
    assert_eq!(encode(|enc| {
        enc.obj_ptr(0, 0)
    }), &[0b11111101]);
    
    assert_eq!(encode(|enc| {
        enc.obj_ptr(0, 1)
    }), &[0b11101000, 1, 0, 0, 0]);

    assert_eq!(encode(|enc| {
        enc.obj_ptr(5, 256)
    }), &[0b11101101, 0, 1, 0, 0]);

    assert_eq!(encode(|enc| {
        enc.obj_ptr(5, u32::MAX as u64)
    }), &[0b11101101, 255, 255, 255, 255]);

    assert_eq!(encode(|enc| {
        enc.obj_ptr(5, u32::MAX as u64 + 1)
    }), &[0b11110101, 0, 0, 0, 0, 1, 0, 0, 0]);

    assert_eq!(encode(|enc| {
        enc.obj_ptr(7, 256)
    }), &[0b11101111, 0, 1, 0, 0]);

    assert_eq!(encode(|enc| {
        enc.obj_ptr(7, u32::MAX as u64)
    }), &[0b11101111, 255, 255, 255, 255]);

    assert_eq!(encode(|enc| {
        enc.obj_ptr(7, u32::MAX as u64 + 1)
    }), &[0b11110111, 0, 0, 0, 0, 1, 0, 0, 0]);

    assert_eq!(encode(|enc| {
        enc.obj_ptr(8, 1)
    }), &[0b11111001, 8, 1, 0, 0, 0]);

    assert_eq!(encode(|enc| {
        enc.obj_ptr(8, 256)
    }), &[0b11111001, 8, 0, 1, 0, 0]);

    assert_eq!(encode(|enc| {
        enc.obj_ptr(8, u32::MAX as u64)
    }), &[0b11111001, 8, 255, 255, 255, 255]);

    assert_eq!(encode(|enc| {
        enc.obj_ptr(8, u32::MAX as u64 + 1)
    }), &[0b11111010, 8, 0, 0, 0, 0, 1, 0, 0, 0]);

    assert_eq!(encode(|enc| {
        enc.obj_ptr(255, 1)
    }), &[0b11111001, 255, 1, 0, 0, 0]);

    assert_eq!(encode(|enc| {
        enc.obj_ptr(255, 256)
    }), &[0b11111001, 255, 0, 1, 0, 0]);

    assert_eq!(encode(|enc| {
        enc.obj_ptr(255, u32::MAX as u64)
    }), &[0b11111001, 255, 255, 255, 255, 255]);

    assert_eq!(encode(|enc| {
        enc.obj_ptr(255, u32::MAX as u64 + 1)
    }), &[0b11111010, 255, 0, 0, 0, 0, 1, 0, 0, 0]);

    assert_eq!(encode(|enc| {
        enc.obj_ptr(256, 1)
    }), &[0b11111011, 0, 1, 1, 0, 0, 0]);

    assert_eq!(encode(|enc| {
        enc.obj_ptr(256, 256)
    }), &[0b11111011, 0, 1, 0, 1, 0, 0]);

    assert_eq!(encode(|enc| {
        enc.obj_ptr(256, u32::MAX as u64)
    }), &[0b11111011, 0, 1, 255, 255, 255, 255]);

    assert_eq!(encode(|enc| {
        enc.obj_ptr(256, u32::MAX as u64 + 1)
    }), &[0b11111100, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0]);

}
