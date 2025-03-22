use super::Range;


pub fn map(val: f32, from: Range, to: Range) -> f32 {
    let t = (val - from.min) / (from.max - from.min);
    t * (to.max - to.min) + to.min
}
