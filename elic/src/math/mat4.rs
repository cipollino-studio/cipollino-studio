
use std::ops::{Add, Mul, Sub};
use crate::math::vec4;

use super::{Vec2, Vec4};

#[derive(Clone, Copy)]
pub struct Mat4 {
    pub x: Vec4,
    pub y: Vec4,
    pub z: Vec4,
    pub w: Vec4,
}

impl Mat4 {

    pub fn scale(scale: Vec2) -> Self {
        Self {
            x: Vec4::X * scale.x,
            y: Vec4::Y * scale.y,
            z: Vec4::Z,
            w: Vec4::W,
        }
    }

    // Copied from glam's source code
    pub fn inverse(&self) -> Self {
        let [m00, m01, m02, m03] = self.x.into();
        let [m10, m11, m12, m13] = self.y.into();
        let [m20, m21, m22, m23] = self.z.into();
        let [m30, m31, m32, m33] = self.w.into();

        let coef00 = m22 * m33 - m32 * m23;
        let coef02 = m12 * m33 - m32 * m13;
        let coef03 = m12 * m23 - m22 * m13;

        let coef04 = m21 * m33 - m31 * m23;
        let coef06 = m11 * m33 - m31 * m13;
        let coef07 = m11 * m23 - m21 * m13;

        let coef08 = m21 * m32 - m31 * m22;
        let coef10 = m11 * m32 - m31 * m12;
        let coef11 = m11 * m22 - m21 * m12;

        let coef12 = m20 * m33 - m30 * m23;
        let coef14 = m10 * m33 - m30 * m13;
        let coef15 = m10 * m23 - m20 * m13;

        let coef16 = m20 * m32 - m30 * m22;
        let coef18 = m10 * m32 - m30 * m12;
        let coef19 = m10 * m22 - m20 * m12;

        let coef20 = m20 * m31 - m30 * m21;
        let coef22 = m10 * m31 - m30 * m11;
        let coef23 = m10 * m21 - m20 * m11;

        let fac0 = vec4(coef00, coef00, coef02, coef03);
        let fac1 = vec4(coef04, coef04, coef06, coef07);
        let fac2 = vec4(coef08, coef08, coef10, coef11);
        let fac3 = vec4(coef12, coef12, coef14, coef15);
        let fac4 = vec4(coef16, coef16, coef18, coef19);
        let fac5 = vec4(coef20, coef20, coef22, coef23);

        let vec0 = vec4(m10, m00, m00, m00);
        let vec1 = vec4(m11, m01, m01, m01);
        let vec2 = vec4(m12, m02, m02, m02);
        let vec3 = vec4(m13, m03, m03, m03);

        let inv0 = vec1.mul(fac0).sub(vec2.mul(fac1)).add(vec3.mul(fac2));
        let inv1 = vec0.mul(fac0).sub(vec2.mul(fac3)).add(vec3.mul(fac4));
        let inv2 = vec0.mul(fac1).sub(vec1.mul(fac3)).add(vec3.mul(fac5));
        let inv3 = vec0.mul(fac2).sub(vec1.mul(fac4)).add(vec2.mul(fac5));

        let sign_a = vec4(1.0, -1.0, 1.0, -1.0);
        let sign_b = vec4(-1.0, 1.0, -1.0, 1.0);

        let inverse = Self {
            x: inv0.mul(sign_a),
            y: inv1.mul(sign_b),
            z: inv2.mul(sign_a),
            w: inv3.mul(sign_b),
        };

        let col0 = vec4(
            inverse.x.x,
            inverse.y.x,
            inverse.z.x,
            inverse.w.x,
        );

        let dot0 = self.x.mul(col0);
        let dot1 = dot0.x + dot0.y + dot0.z + dot0.w;

        assert!(dot1 != 0.0, "matrix cannot be inverted");

        let rcp_det = dot1.recip();
        inverse.mul(rcp_det)
    }

    pub fn orthographic(l: f32, r: f32, b: f32, t: f32) -> Self {
        let dx = 2.0 / (r - l);
        let dy = 2.0 / (t - b);
        let tx = -(r + l) / (r - l);
        let ty = -(t + b) / (t - b);

        Self {
            x: vec4(dx, 0.0, 0.0, 0.0),
            y: vec4(0.0, dy, 0.0, 0.0),
            z: vec4(0.0, 0.0, -1.0, 0.0),
            w: vec4(tx, ty, 0.0, 1.0),
        }
    }

}

impl Mul<f32> for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: f32) -> Mat4 {
        Mat4 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Vec4 {
        rhs.x * self.x + rhs.y * self.y + rhs.z * self.z + rhs.w * self.w
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Mat4 {
        Mat4 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            w: self * rhs.w
        }
    }
}

impl From<Mat4> for [[f32; 4]; 4] {

    fn from(mat: Mat4) -> Self {
        [
            mat.x.into(),
            mat.y.into(),
            mat.z.into(),
            mat.w.into()
        ]
    }

}
