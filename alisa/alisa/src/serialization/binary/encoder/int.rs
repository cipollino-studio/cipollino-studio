
use super::{Encoder, Result};

impl<'writer, W: std::io::Write> Encoder<'writer, W> {

    fn write_i8_value(&mut self, val: i8) -> Result {
        self.write_byte(0b11000110)?;
        self.write_bytes(&val.to_le_bytes())
    }

    fn write_i16_value(&mut self, val: i16) -> Result {
        self.write_byte(0b11000111)?;
        self.write_bytes(&val.to_le_bytes())
    }

    fn write_i32_value(&mut self, val: i32) -> Result {
        self.write_byte(0b11001000)?;
        self.write_bytes(&val.to_le_bytes())
    }

    fn write_i64_value(&mut self, val: i64) -> Result {
        self.write_byte(0b11001001)?;
        self.write_bytes(&val.to_le_bytes())
    }

    pub fn i8(&mut self, val: i8) -> Result {
        if val >= 0 {
            return self.write_byte(val as u8);
        }
        self.write_i8_value(val)
    }

    pub fn i16(&mut self, val: i16) -> Result {
        if val >= i8::MIN as i16 && val <= i8::MAX as i16 {
            return self.i8(val as i8);
        } 
        self.write_i16_value(val)
    }

    pub fn i32(&mut self, val: i32) -> Result {
        if val >= i16::MIN as i32 && val <= i16::MAX as i32 {
            return self.i16(val as i16);
        } 
        self.write_i32_value(val)
    }

    pub fn i64(&mut self, val: i64) -> Result {
        if val >= i32::MIN as i64 && val <= i32::MAX as i64 {
            return self.i32(val as i32);
        } 
        self.write_i64_value(val)
    }

}
