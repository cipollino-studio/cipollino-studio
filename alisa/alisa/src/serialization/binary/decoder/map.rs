
use super::Decoder;

impl Decoder<'_> {

    pub fn is_map(&self) -> bool {
        let Some(first) = self.peek() else { return false; };
        first & 0b11110000 == 0b10110000 ||
        first == 0b11010101 ||
        first == 0b11010110 ||
        first == 0b11010111
    }

    pub fn read_map_length(&mut self) -> Option<u32> {
        let first = self.peek()?;
        match first {
            _ if first & 0b11110000 == 0b10110000 => {
                self.read()?;
                Some((first & 0b00001111) as u32)
            },
            0b11010101 => {
                self.read()?;
                Some(self.read()? as u32)
            },
            0b11010110 => {
                self.read()?;
                Some(u16::from_le_bytes(self.read_array()?) as u32)
            },
            0b11010111 => {
                self.read()?;
                Some(u32::from_le_bytes(self.read_array()?))
            },
            _ => {
                None
            }
        }
    }

    pub fn read_map_field_name(&mut self) -> Option<&str> {
        self.read_symbol()
    }

}
