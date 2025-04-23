
mod bool;
mod uint;
mod int;
mod float;
mod string;
mod binary;
mod obj_ptr;
mod enums;
mod array;
mod map;
mod skip;

mod parser;
pub use parser::*;

pub struct Decoder<'data> {
    data: &'data [u8]
}

impl<'data> Decoder<'data> {

    pub fn new(data: &'data [u8]) -> Self {
        Self {
            data
        }
    }

    pub fn remaining_bytes(&self) -> &[u8] {
        self.data
    }

    pub fn done(&self) -> bool {
        self.remaining_bytes().is_empty()
    }

    fn peek(&self) -> Option<u8> {
        self.data.first().copied()
    }

    fn read(&mut self) -> Option<u8> {
        let first = self.data.first()?;
        self.data = &self.data[1..];
        Some(*first)
    }

    fn read_slice(&mut self, n: usize) -> Option<&[u8]> {
        if self.data.len() < n {
            return None;
        }
        let read = &self.data[..n];
        self.data = &self.data[n..];
        Some(read)
    }

    fn read_array<const N: usize>(&mut self) -> Option<[u8; N]> {
        let mut result = [0; N];
        for (i, byte) in self.read_slice(N)?.iter().enumerate() {
            result[i] = *byte;
        }
        Some(result)
    }

    fn read_symbol(&mut self) -> Option<&str> {
        let first = self.read()?;
        let length = match first {
            _ if first & 0b10000000 == 0b00000000 => {
                first as usize
            },
            0b10000000 => {
                self.read()? as usize
            },
            0b10000001 => {
                u16::from_le_bytes(self.read_array()?) as usize
            },
            0b10000010 => {
                u32::from_le_bytes(self.read_array()?) as usize
            },
            _ => {
                return None;
            }
        };
        let bytes = self.read_slice(length)?;
        std::str::from_utf8(bytes).ok()
    }

}
