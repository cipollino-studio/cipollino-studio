
use super::Decoder;

impl Decoder<'_> {

    pub fn is_obj_ptr(&self) -> bool {
        let Some(first) = self.peek() else { return false; };
        first & 0b11111000 == 0b11101000 ||
        first & 0b11111000 == 0b11110000 || 
        first == 0b11111001 ||
        first == 0b11111010 ||
        first == 0b11111011 ||
        first == 0b11111100 ||
        first == 0b11111101
    }

    pub fn read_obj_ptr(&mut self) -> Option<(u16, u64)> {
        let first = self.peek()?;
        if first == 0b11111101 {
            self.read()?;
            return Some((u16::MAX, 0));
        }

        let (obj_type, large) = match first {
            _ if first & 0b11111000 == 0b11101000 => {
                self.read()?;
                ((first & 0b00000111) as u16, false)
            },
            _ if first & 0b11111000 == 0b11110000 => {
                self.read()?;
                ((first & 0b00000111) as u16, true)
            },
            0b11111001 => {
                self.read()?;
                (self.read()? as u16, false)
            },
            0b11111010 => {
                self.read()?;
                (self.read()? as u16, true)
            },
            0b11111011 => {
                self.read()?;
                (u16::from_le_bytes(self.read_array()?), false)
            },
            0b11111100 => {
                self.read()?;
                (u16::from_le_bytes(self.read_array()?), true)
            },
            _ => {
                return None;
            }
        };

        let key = if large {
            u64::from_le_bytes(self.read_array()?)
        } else {
            u32::from_le_bytes(self.read_array()?) as u64
        };

        Some((obj_type, key))
    }

}
