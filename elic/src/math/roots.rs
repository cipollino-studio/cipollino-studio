
use crate::SmallArr;

/// Solves ax^2 + bx + c = 0
pub fn roots_quadratic(a: f32, b: f32, c: f32) -> SmallArr<f32, 2> {
    let d = b * b - 4.0 * a * c;
    if d < 0.0 {
        return SmallArr::empty();
    }
    if d == 0.0 {
        return SmallArr::from_slice(&[
            -b / (2.0 * a)
        ]);
    }
    let d_sqrt = d.sqrt();
    return SmallArr::from_slice(&[
        (-b - d_sqrt) / (2.0 * a),
        (-b + d_sqrt) / (2.0 * a),
    ])
}
