
use super::Decoder;

fn is_positive_int(first: u8) -> bool {
    first & 0b10000000 == 0
}

fn is_i8(first: u8) -> bool {
    is_positive_int(first) || first == 0b11000110
}

fn is_i16(first: u8) -> bool {
    is_i8(first) || first == 0b11000111
}

fn is_i32(first: u8) -> bool {
    is_i16(first) || first == 0b11001000
}

fn is_i64(first: u8) -> bool {
    is_i32(first) || first == 0b11001001
}

impl Decoder<'_> {

    pub fn is_i8(&mut self) -> bool {
        self.peek().map(is_i8).unwrap_or(false)
    }

    pub fn is_i16(&mut self) -> bool {
        self.peek().map(is_i16).unwrap_or(false)
    }

    pub fn is_i32(&mut self) -> bool {
        self.peek().map(is_i32).unwrap_or(false)
    }

    pub fn is_i64(&mut self) -> bool {
        self.peek().map(is_i64).unwrap_or(false)
    }

    pub(crate) fn read_positive_int(&mut self) -> Option<u8> {
        let first = self.peek()?;

        // Positive int
        if first & 0b10000000 == 0 {
            self.read();
            return Some(first as u8);
        }

        None
    }

    pub fn read_i8(&mut self) -> Option<i8> {
        let first = self.peek()?;

        // Positive int
        if first & 0b10000000 == 0 {
            self.read();
            return Some(first as i8);
        }

        if first == 0b11000110 {
            self.read(); // First byte
            return self.read_array().map(i8::from_le_bytes);
        }

        None
    }

    pub fn read_i16(&mut self) -> Option<i16> {
        if let Some(x) = self.read_i8() {
            return Some(x as i16);
        } 

        if self.peek()? == 0b11000111 {
            self.read(); // First byte
            return self.read_array().map(i16::from_le_bytes);
        }

        None
    }

    pub fn read_i32(&mut self) -> Option<i32> {
        if let Some(x) = self.read_i16() {
            return Some(x as i32);
        } 

        if self.peek()? == 0b11001000 {
            self.read(); // First byte
            return self.read_array().map(i32::from_le_bytes);
        }

        None
    }

    pub fn read_i64(&mut self) -> Option<i64> {
        if let Some(x) = self.read_i32() {
            return Some(x as i64);
        } 

        if self.peek()? == 0b11001001 {
            self.read(); // First byte
            return self.read_array().map(i64::from_le_bytes);
        }

        None
    }

}
