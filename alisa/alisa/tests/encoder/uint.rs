
use std::{u16, u64};

use crate::encode;

#[test]
fn uint8() {

    assert_eq!(encode(|enc| {
        enc.u8(5)
    }), &[5]);

    assert_eq!(encode(|enc| {
        enc.u8(127)
    }), &[127]);

    assert_eq!(encode(|enc| {
        enc.u8(128)
    }), &[0b11000010, 128]);

    assert_eq!(encode(|enc| {
        enc.u8(255)
    }), &[0b11000010, 255]);

}

#[test]
fn uint16() {

    assert_eq!(encode(|enc| {
        enc.u16(5)
    }), &[5]);

    assert_eq!(encode(|enc| {
        enc.u16(127)
    }), &[127]);

    assert_eq!(encode(|enc| {
        enc.u16(128)
    }), &[0b11000010, 128]);

    assert_eq!(encode(|enc| {
        enc.u16(255)
    }), &[0b11000010, 255]);

    assert_eq!(encode(|enc| {
        enc.u16(256)
    }), &[0b11000011, 0, 1]);

    assert_eq!(encode(|enc| {
        enc.u16(u16::MAX)
    }), &[0b11000011, 255, 255]);

}

#[test]
fn uint32() {

    assert_eq!(encode(|enc| {
        enc.u32(5)
    }), &[5]);

    assert_eq!(encode(|enc| {
        enc.u32(127)
    }), &[127]);

    assert_eq!(encode(|enc| {
        enc.u32(128)
    }), &[0b11000010, 128]);
    
    assert_eq!(encode(|enc| {
        enc.u32(255)
    }), &[0b11000010, 255]);

    assert_eq!(encode(|enc| {
        enc.u32(256)
    }), &[0b11000011, 0, 1]);

    assert_eq!(encode(|enc| {
        enc.u32(u16::MAX as u32)
    }), &[0b11000011, 255, 255]);

    assert_eq!(encode(|enc| {
        enc.u32(u16::MAX as u32 + 1)
    }), &[0b11000100, 0, 0, 1, 0]);

    assert_eq!(encode(|enc| {
        enc.u32(u32::MAX)
    }), &[0b11000100, 255, 255, 255, 255]);

}

#[test]
fn uint64() {

    assert_eq!(encode(|enc| {
        enc.u64(5)
    }), &[5]);

    assert_eq!(encode(|enc| {
        enc.u64(127)
    }), &[127]);

    assert_eq!(encode(|enc| {
        enc.u64(128)
    }), &[0b11000010, 128]);
    
    assert_eq!(encode(|enc| {
        enc.u64(255)
    }), &[0b11000010, 255]);

    assert_eq!(encode(|enc| {
        enc.u64(256)
    }), &[0b11000011, 0, 1]);

    assert_eq!(encode(|enc| {
        enc.u64(u16::MAX as u64)
    }), &[0b11000011, 255, 255]);

    assert_eq!(encode(|enc| {
        enc.u64(u16::MAX as u64 + 1)
    }), &[0b11000100, 0, 0, 1, 0]);

    assert_eq!(encode(|enc| {
        enc.u64(u32::MAX as u64)
    }), &[0b11000100, 255, 255, 255, 255]);

    assert_eq!(encode(|enc| {
        enc.u64(u32::MAX as u64 + 1)
    }), &[0b11000101, 0, 0, 0, 0, 1, 0, 0, 0]);

    assert_eq!(encode(|enc| {
        enc.u64(u64::MAX)
    }), &[0b11000101, 255, 255, 255, 255, 255, 255, 255, 255]);

}
