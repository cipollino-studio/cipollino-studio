
use super::{Encoder, Result};

impl<'writer, W: std::io::Write> Encoder<'writer, W> {

    pub fn string(&mut self, string: &str) -> Result {
        let bytes = string.as_bytes();
        let n_bytes = bytes.len();
        if n_bytes < (1 << 5) {
            self.write_byte(0b10000000 | (n_bytes as u8))?;
            return self.write_bytes(bytes);
        }
        if n_bytes <= u8::MAX as usize {
            self.write_byte(0b11001100)?;
            self.write_byte(n_bytes as u8)?; 
            return self.write_bytes(bytes);
        }
        if n_bytes <= u16::MAX as usize {
            self.write_byte(0b11001101)?;
            self.write_bytes(&(n_bytes as u16).to_le_bytes())?; 
            return self.write_bytes(bytes);
        }
        // Cap the string length to fit in a U32
        // In theory this could make the string invalid UTF8, but this is a very rare case
        let n_bytes = n_bytes.min(u32::MAX as usize); 
        self.write_byte(0b11001110)?;
        self.write_bytes(&(n_bytes as u32).to_le_bytes())?;
        self.write_bytes(&bytes[0..n_bytes])
    }

}
