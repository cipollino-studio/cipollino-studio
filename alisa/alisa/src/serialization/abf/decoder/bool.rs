
use super::Decoder;

impl Decoder<'_> {

    pub fn is_bool(&mut self) -> bool {
        let Some(curr) = self.peek() else { return false; };
        curr == 0b11000000 || curr == 0b11000001
    }

    pub fn read_bool(&mut self) -> Option<bool> {
        if !self.is_bool() {
            return None;
        }
        let byte = self.read()?;
        Some(byte == 0b11000001)
    }

}
