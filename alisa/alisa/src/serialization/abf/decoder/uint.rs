
use super::Decoder;

fn is_positive_int(first: u8) -> bool {
    first & 0b10000000 == 0
}

fn is_u8(first: u8) -> bool {
    is_positive_int(first) || first == 0b11000010
}

fn is_u16(first: u8) -> bool {
    is_u8(first) || first == 0b11000011
}

fn is_u32(first: u8) -> bool {
    is_u16(first) || first == 0b11000100
}

fn is_u64(first: u8) -> bool {
    is_u32(first) || first == 0b11000101
}

impl Decoder<'_> {

    pub fn is_u8(&mut self) -> bool {
        self.peek().map(is_u8).unwrap_or(false)
    }

    pub fn is_u16(&mut self) -> bool {
        self.peek().map(is_u16).unwrap_or(false)
    }

    pub fn is_u32(&mut self) -> bool {
        self.peek().map(is_u32).unwrap_or(false)
    }

    pub fn is_u64(&mut self) -> bool {
        self.peek().map(is_u64).unwrap_or(false)
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        let first = self.peek()?;

        // Positive int
        if first & 0b10000000 == 0 {
            self.read();
            return Some(first);
        }

        if first == 0b11000010 {
            self.read(); // First byte
            return self.read();
        }

        None
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        if let Some(x) = self.read_u8() {
            return Some(x as u16);
        } 

        if self.peek()? == 0b11000011 {
            self.read(); // First byte
            return self.read_array().map(u16::from_le_bytes);
        }

        None
    }

    pub fn read_u32(&mut self) -> Option<u32> {
        if let Some(x) = self.read_u16() {
            return Some(x as u32);
        } 

        if self.peek()? == 0b11000100 {
            self.read(); // First byte
            return self.read_array().map(u32::from_le_bytes);
        }

        None
    }

    pub fn read_u64(&mut self) -> Option<u64> {
        if let Some(x) = self.read_u32() {
            return Some(x as u64);
        } 

        if self.peek()? == 0b11000101 {
            self.read(); // First byte
            return self.read_array().map(u64::from_le_bytes);
        }

        None
    }

}
