
use super::{Encoder, Result};

impl<'writer, W: std::io::Write> Encoder<'writer, W> {

    pub fn f32(&mut self, val: f32) -> Result {
        self.write_byte(0b11001010)?;
        self.write_bytes(&val.to_le_bytes())
    }

    pub fn f64(&mut self, val: f64) -> Result {
        self.write_byte(0b11001011)?;
        self.write_bytes(&val.to_le_bytes())
    }

}
