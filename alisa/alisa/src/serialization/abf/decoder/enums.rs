
use super::Decoder;

impl Decoder<'_> {

    pub fn is_indexed_unit_enum(&self) -> bool {
        let Some(first) = self.peek() else { return false; };
        first & 0b11111000 == 0b11011000
    }

    pub fn is_indexed_enum(&self) -> bool {
        let Some(first) = self.peek() else { return false; };
        first & 0b11111000 == 0b11100000
    }

    pub fn is_named_unit_enum(&self) -> bool {
        let Some(first) = self.peek() else { return false; };
        first == 0b11111110
    }

    pub fn is_named_enum(&self) -> bool {
        let Some(first) = self.peek() else { return false; };
        first == 0b11111111
    }

    pub fn read_indexed_unit_enum(&mut self) -> Option<u8> {
        let first = self.peek()?;
        if first & 0b11111000 != 0b11011000 {
            return None;
        }
        self.read()?;
        Some(first & 0b00000111)
    }

    pub fn read_indexed_enum(&mut self) -> Option<u8> {
        let first = self.peek()?;
        if first & 0b11111000 != 0b11100000 {
            return None;
        }
        self.read()?;
        Some(first & 0b00000111)
    }

    pub fn read_named_unit_enum(&mut self) -> Option<&str> {
        let first = self.peek()?;
        if first != 0b11111110 {
            return None;
        }
        self.read()?;
        self.read_symbol()
    }

    pub fn read_named_enum(&mut self) -> Option<&str> {
        let first = self.peek()?;
        if first != 0b11111111 {
            return None;
        }
        self.read()?;
        self.read_symbol()
    }

}
