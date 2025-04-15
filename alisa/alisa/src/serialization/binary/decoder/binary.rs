
use super::Decoder;

impl Decoder<'_> {

    pub fn is_binary(&self) -> bool {
        let Some(first) = self.peek() else { return false; };
        first == 0b11001111 || first == 0b11010000 || first == 0b11010001 
    }
    
    pub fn read_binary(&mut self) -> Option<&[u8]> {
        let first = self.peek()?;
        let length = if first == 0b11001111 {
            self.read()?;
            self.read()? as usize
        } else if first == 0b11010000 {
            self.read()?;
            u16::from_le_bytes(self.read_array()?) as usize
        } else if first == 0b11010001 {
            self.read()?;
            u32::from_le_bytes(self.read_array()?) as usize
        } else {
            return None;
        };
        self.read_slice(length)
    }

}
