use std::ops::{Add, Mul, Neg, Sub};

pub use crate::ffi::Vector2;

impl Vector2 {
    pub const ZERO: Vector2 = Vector2 { x: 0., y: 0. };

    /// Creates a `Vector2` where both `x` and `y` are equal to the given `size`
    pub fn square(size: f32) -> Self {
        Self { x: size, y: size }
    }
}

impl Add for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Self) -> Self::Output {
        return Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}
impl Sub for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Self) -> Self::Output {
        return Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Neg for Vector2 {
    type Output = Vector2;

    fn neg(self) -> Self::Output {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}
