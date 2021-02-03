use std::ops::{Add, Mul, Neg};


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32
}

impl Vec2 {
    #[inline]
    pub fn length(&self) -> f32 {
        ((self.x * self.x + self.y * self.y) as f32).sqrt()
    }

    pub fn max(&self, other: &Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y)
        }
    }

    pub fn min(&self, other: &Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y)
        }
    }

    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs()
        }
    }
}

impl From<(i32, i32)> for Vec2 {
    fn from(arg: (i32, i32)) -> Self {
        Self {
            x: arg.0,
            y: arg.1
        }
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Add<i32> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs
        }
    }
}

impl Add<f32> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: f32) -> Self::Output {
        self.add(rhs as i32)
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs as i32,
            y: self.y * rhs as i32
        }
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = i32;

    fn mul(self, rhs: Vec2) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}