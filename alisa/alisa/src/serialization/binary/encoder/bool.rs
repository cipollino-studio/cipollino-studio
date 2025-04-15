
use super::{Encoder, Result};

impl<'writer, W: std::io::Write> Encoder<'writer, W> {

    pub fn boolean(&mut self, val: bool) -> Result {
        if val {
            self.write_byte(0b11000001) 
        } else {
            self.write_byte(0b11000000) 
        }
    }

}
