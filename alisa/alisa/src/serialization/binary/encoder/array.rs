
use super::{Encoder, Result};

impl<'writer, W: std::io::Write> Encoder<'writer, W> {

    pub fn array(&mut self, length: u32) -> Result {
        if length < (1 << 4) {
            self.write_byte(0b10100000 | (length as u8))?
        } else if length <= u8::MAX as u32 {
            self.write_byte(0b11010010)?;
            self.write_byte(length as u8)?;
        } else if length <= u16::MAX as u32 {
            self.write_byte(0b11010011)?;
            self.write_bytes(&(length as u16).to_le_bytes())?;
        } else {
            self.write_byte(0b11010100)?;
            self.write_bytes(&length.to_le_bytes())?;
        }
        Ok(())
    } 

}
