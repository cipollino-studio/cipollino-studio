
use super::Decoder;

impl Decoder<'_> {

    fn skip_symbol(&mut self) {
        let Some(first) = self.read() else { return; };
        let length = match first {
            _ if first & 0b10000000 == 0b00000000 => {
                first as usize
            },
            0b10000000 => {
                let Some(length) = self.read() else { return; };
                length as usize
            },
            0b10000001 => {
                let Some(length) = self.read_array() else { return; };
                u16::from_le_bytes(length) as usize
            },
            0b10000010 => {
                let Some(length) = self.read_array() else { return; };
                u32::from_le_bytes(length) as usize
            },
            _ => {
                return
            }
        };
        self.read_slice(length);
    }

    pub fn skip_value(&mut self) {
        let Some(first) = self.read() else { return; };
        match first {
            // Small string
            _ if first & 0b11100000 == 0b10000000 => {
                let length = (first & 0b00011111) as usize;
                self.read_slice(length);
            },
            // Small array
            _ if first & 0b11110000 == 0b10100000 => {
                let length = (first & 0b00001111) as usize;
                for _ in 0..length {
                    self.skip_value();
                }
            },
            // Small map
            _ if first & 0b11110000 == 0b10110000 => {
                let length = (first & 0b00001111) as usize;
                for _ in 0..length {
                    self.skip_symbol();
                    self.skip_value();
                }
            },
            // U8, I8
            0b11000010 | 0b11000110 => {
                self.read();
            },
            // U16, I16
            0b11000011 | 0b11000111 => {
                self.read_array::<2>();
            },
            // U32, I32, F32
            0b11000100 | 0b11001000 | 0b11001010 => {
                self.read_array::<4>();
            },
            // U64, I64, F64
            0b11000101 | 0b11001001 | 0b11001011 => {
                self.read_array::<8>();
            },
            // String U8, Binary U8
            0b11001100 | 0b11001111 => {
                let Some(length) = self.read() else { return; };
                self.read_slice(length as usize);
            },
            // String U16, Binary U16
            0b11001101 | 0b11010000 => {
                let Some(length) = self.read_array() else { return; };
                self.read_slice(u16::from_le_bytes(length) as usize);
            },
            // String U32, Binary U32
            0b11001110 | 0b11010001 => {
                let Some(length) = self.read_array() else { return; };
                self.read_slice(u32::from_le_bytes(length) as usize);
            },
            // Array U8
            0b11010010 => {
                let Some(length) = self.read() else { return; };
                for _ in 0..length {
                    self.skip_value();
                }
            },
            // Array U16
            0b11010011 => {
                let Some(length) = self.read_array() else { return; };
                for _ in 0..u16::from_le_bytes(length) {
                    self.skip_value();
                }
            },
            // Array U32
            0b11010100 => {
                let Some(length) = self.read_array() else { return; };
                for _ in 0..u32::from_le_bytes(length) {
                    self.skip_value();
                }
            },
            // Map U8
            0b11010101 => {
                let Some(length) = self.read() else { return; };
                for _ in 0..length {
                    self.skip_symbol();
                    self.skip_value();
                }
            },
            // Map U16
            0b11010110 => {
                let Some(length) = self.read_array() else { return; };
                for _ in 0..u16::from_le_bytes(length) {
                    self.skip_symbol();
                    self.skip_value();
                }
            },
            // Map U32
            0b11010111 => {
                let Some(length) = self.read_array() else { return; };
                for _ in 0..u32::from_le_bytes(length) {
                    self.skip_symbol();
                    self.skip_value();
                }
            },
            // Indexed enum
            _ if first & 0b11111000 == 0b11100000 => {
                self.skip_value();
            }
            // Small Obj Ptr with small type
            _ if first & 0b11111000 == 0b11101000 => {
                self.read_array::<4>();
            }
            // Large Obj Ptr with small type
            _ if first & 0b11111000 == 0b11110000 => {
                self.read_array::<8>();
            },
            // Small Obj Ptr with U8 type
            0b11111001 => {
                self.read_array::<5>();
            }
            // Large Obj Ptr with U8 type
            0b11111010 => {
                self.read_array::<9>();
            }
            // Small Obj Ptr with U16 type
            0b11111011 => {
                self.read_array::<6>();
            }
            // Large Obj Ptr with U16 type
            0b11111100 => {
                self.read_array::<10>();
            },
            // Named unit enum
            0b11111110 => {
                self.skip_symbol();
            },
            // Named enum
            0b11111111 => {
                self.skip_symbol();
                self.skip_value();
            }
            _ => {}
        }
    }

}
