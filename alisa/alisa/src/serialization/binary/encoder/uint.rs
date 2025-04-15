
use super::{Encoder, Result};

impl<'writer, W: std::io::Write> Encoder<'writer, W> {

    fn write_u8_value(&mut self, val: u8) -> Result {
        self.write_byte(0b11000010)?;
        self.write_byte(val)
    }

    fn write_u16_value(&mut self, val: u16) -> Result {
        self.write_byte(0b11000011)?;
        self.write_bytes(&val.to_le_bytes())
    }

    fn write_u32_value(&mut self, val: u32) -> Result {
        self.write_byte(0b11000100)?;
        self.write_bytes(&val.to_le_bytes())
    }

    fn write_u64_value(&mut self, val: u64) -> Result {
        self.write_byte(0b11000101)?;
        self.write_bytes(&val.to_le_bytes())
    }

    pub fn u8(&mut self, val: u8) -> Result {
        if val < (1 << 7) {
            return self.write_byte(val);
        }
        self.write_u8_value(val)
    }

    pub fn u16(&mut self, val: u16) -> Result {
        if val <= u8::MAX as u16 {
            return self.u8(val as u8);
        } 
        self.write_u16_value(val)
    }

    pub fn u32(&mut self, val: u32) -> Result {
        if val <= u16::MAX as u32 {
            return self.u16(val as u16);
        } 
        self.write_u32_value(val)
    }

    pub fn u64(&mut self, val: u64) -> Result {
        if val <= u32::MAX as u64 {
            return self.u32(val as u32);
        } 
        self.write_u64_value(val)
    }

}
