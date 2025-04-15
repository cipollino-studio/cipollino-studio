
use super::Decoder;

impl Decoder<'_> {

    pub fn is_string(&self) -> bool {
        let Some(first) = self.peek() else { return false; };
        first & 0b11100000 == 0b10000000 || first == 0b11001100 || first == 0b11001101 || first == 0b11001110
    }
    
    pub fn read_string(&mut self) -> Option<&str> {
        let first = self.peek()?;
        let length = if first & 0b11100000 == 0b10000000 {
            self.read()?;
            (first & 0b00011111) as usize
        } else if first == 0b11001100 {
            self.read()?;
            self.read()? as usize
        } else if first == 0b11001101 {
            self.read()?;
            u16::from_le_bytes(self.read_array()?) as usize
        } else if first == 0b11001110 {
            self.read()?;
            u32::from_le_bytes(self.read_array()?) as usize
        } else {
            return None;
        };
        let str_bytes = self.read_slice(length)?;
        std::str::from_utf8(str_bytes).ok()
    }

}
