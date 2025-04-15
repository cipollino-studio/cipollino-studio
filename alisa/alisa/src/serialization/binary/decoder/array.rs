
use super::Decoder;

impl Decoder<'_> {

    pub fn is_array(&self) -> bool {
        let Some(first) = self.peek() else { return false; };
        first & 0b11110000 == 0b10100000 ||
        first == 0b11010010 ||
        first == 0b11010011 ||
        first == 0b11010100
    }

    pub fn read_array_length(&mut self) -> Option<u32> {
        let first = self.peek()?;
        match first {
            _ if first & 0b11110000 == 0b10100000 => {
                self.read()?;
                Some((first & 0b00001111) as u32)
            },
            0b11010010 => {
                self.read()?;
                Some(self.read()? as u32)
            },
            0b11010011 => {
                self.read()?;
                Some(u16::from_le_bytes(self.read_array()?) as u32)
            },
            0b11010100 => {
                self.read()?;
                Some(u32::from_le_bytes(self.read_array()?))
            },
            _ => {
                None
            }
        }
    }

}
