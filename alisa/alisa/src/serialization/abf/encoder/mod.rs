
mod bool;
mod uint;
mod int;
mod float;
mod string;
mod binary;
mod obj_ptr;
mod array;
mod map;
mod enums;

type Result = std::io::Result<()>;

pub struct Encoder<'writer, W: std::io::Write> {
    writer: &'writer mut W
}

impl<'writer, W: std::io::Write> Encoder<'writer, W> {

    pub fn new(writer: &'writer mut W) -> Self {
        Self {
            writer
        }
    }

    fn write_bytes(&mut self, bytes: &[u8]) -> Result {
        self.writer.write(bytes).map(|_| ())
    }

    fn write_byte(&mut self, byte: u8) -> Result {
        self.write_bytes(&[byte])
    }

    fn write_symbol(&mut self, symbol: &str) -> Result {
        let bytes = symbol.as_bytes(); 
        let n_bytes = bytes.len();
        if n_bytes < (1 << 7) {
            self.write_byte(n_bytes as u8)?;
            return self.write_bytes(bytes);
        }
        if n_bytes <= u8::MAX as usize {
            self.write_byte(0b10000000)?;
            self.write_byte(n_bytes as u8)?;
            return self.write_bytes(bytes);
        }
        if n_bytes <= u16::MAX as usize {
            self.write_byte(0b10000001)?;
            self.write_bytes(&(n_bytes as u16).to_le_bytes())?;
            return self.write_bytes(bytes);
        }
        // Cap the string length to fit in a U32
        // In theory this could make the string invalid UTF8, but this is a very rare case
        let n_bytes = n_bytes.min(u32::MAX as usize); 
        self.write_byte(0b10000010)?;
        self.write_bytes(&(n_bytes as u32).to_le_bytes())?;
        self.write_bytes(&bytes[0..n_bytes])
    }

}
