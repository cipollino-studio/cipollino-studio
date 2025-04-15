
use super::{Encoder, Result};

impl<'writer, W: std::io::Write> Encoder<'writer, W> {

    pub fn indexed_unit_enum(&mut self, idx: u8) -> Result {
        if idx >= (1 << 3) {
            panic!("enum variant must be less than 8.");
        }
        self.write_byte(0b11011000 | idx)
    }

    pub fn indexed_enum<F: FnOnce(&mut Encoder<'writer, W>) -> Result>(&mut self, idx: u8, data: F) -> Result {
        if idx >= (1 << 3) {
            panic!("enum variant must be less than 8.");
        }
        self.write_byte(0b11100000 | idx)?;
        data(self)
    }

    pub fn named_unit_enum(&mut self, name: &str) -> Result {
        self.write_byte(0b11111110)?;
        self.write_symbol(name)
    }

    pub fn named_enum<F: FnOnce(&mut Encoder<'writer, W>) -> Result>(&mut self, name: &str, data: F) -> Result {
        self.write_byte(0b11111111)?;
        self.write_symbol(name)?;
        data(self)
    }

}
