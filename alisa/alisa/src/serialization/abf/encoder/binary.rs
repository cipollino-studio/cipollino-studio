
use super::{Encoder, Result};

impl<'writer, W: std::io::Write> Encoder<'writer, W> {

    pub fn binary(&mut self, bytes: &[u8]) -> Result {
        let n_bytes = bytes.len();
        if n_bytes <= u8::MAX as usize {
            self.write_byte(0b11001111)?;
            self.write_byte(n_bytes as u8)?; 
            return self.write_bytes(bytes);
        }
        if n_bytes <= u16::MAX as usize {
            self.write_byte(0b11010000)?;
            self.write_bytes(&(n_bytes as u16).to_le_bytes())?; 
            return self.write_bytes(bytes);
        }
        // Cap the binary length to fit in a U32
        let n_bytes = n_bytes.min(u32::MAX as usize); 
        self.write_byte(0b11010001)?;
        self.write_bytes(&(n_bytes as u32).to_le_bytes())?;
        self.write_bytes(&bytes[0..n_bytes])
    }

}
