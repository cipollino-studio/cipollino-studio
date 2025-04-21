
use super::{Encoder, Result};

impl<'writer, W: std::io::Write> Encoder<'writer, W> {

    pub fn null_ptr(&mut self) -> Result {
        self.write_byte(0b11111101)
    } 

    pub fn obj_ptr(&mut self, obj_type: u16, key: u64) -> Result {
        if key == 0 {
            return self.null_ptr();
        }

        let large = key > u32::MAX as u64;

        if obj_type < (1 << 3) {
            let obj_type = obj_type as u8;
            self.write_byte(if large { 0b11110000 } else { 0b11101000 } | obj_type)?;
        } else if obj_type <= u8::MAX as u16 {
            self.write_byte(if large { 0b11111010 } else { 0b11111001 })?;
            self.write_byte(obj_type as u8)?;
        } else {
            self.write_byte(if large { 0b11111100 } else { 0b11111011 })?;
            self.write_bytes(&obj_type.to_le_bytes())?;
        }

        if large {
            self.write_bytes(&key.to_le_bytes())
        } else {
            self.write_bytes(&(key as u32).to_le_bytes())
        }
    }

}
