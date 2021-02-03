use std::ops::{Add, Neg};


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32
}

impl Vec2 {
    #[inline]
    pub fn length(&self) -> i32 {
        ((self.x * self.x + self.y * self.y) as f32).sqrt() as i32
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

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y
        }
    }
}