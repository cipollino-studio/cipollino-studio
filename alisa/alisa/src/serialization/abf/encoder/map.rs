
use super::{Encoder, Result};

impl<'writer, W: std::io::Write> Encoder<'writer, W> {

    pub fn map(&mut self, length: u32) -> Result {
        if length < (1 << 4) {
            self.write_byte(0b10110000 | (length as u8))?;
        } else if length <= u8::MAX as u32 {
            self.write_byte(0b11010101)?;
            self.write_byte(length as u8)?;
        } else if length <= u16::MAX as u32 {
            self.write_byte(0b11010110)?;
            self.write_bytes(&(length as u16).to_le_bytes())?;
        } else {
            self.write_byte(0b11010111)?;
            self.write_bytes(&length.to_le_bytes())?;
        }
        Ok(())
    } 

    pub fn map_field<F: FnOnce(&mut Encoder<'writer, W>) -> Result>(&mut self, name: &str, data: F) -> Result {
        self.write_symbol(name)?;
        data(self)
    }

}
