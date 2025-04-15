
use super::Decoder;

impl Decoder<'_> {

    pub fn is_f32(&self) -> bool {
        let Some(first) = self.peek() else { return false; };
        first == 0b11001010
    }

    pub fn is_f64(&self) -> bool {
        let Some(first) = self.peek() else { return false; };
        first == 0b11001011
    }

    pub fn read_f32(&mut self) -> Option<f32> {
        if !self.is_f32() {
            return None;
        }
        self.read()?;
        self.read_array().map(f32::from_le_bytes)
    }

    pub fn read_f64(&mut self) -> Option<f64> {
        if !self.is_f64() {
            return None;
        }
        self.read()?;
        self.read_array().map(f64::from_le_bytes)
    }

}
