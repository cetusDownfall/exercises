use piston_window::Size;
use std::ops::{Add, AddAssign, Sub, SubAssign, Rem, RemAssign, Mul, MulAssign, Div, DivAssign};
#[derive(Copy, Clone)]
pub struct Veloc {
    pub x: f64,
    pub y: f64,
}
impl Veloc {
    pub fn hypo(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn dist(self, other: Veloc) -> f64 {
        Veloc::hypo(self - other)
    }
}
impl Add for Veloc {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Veloc {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl AddAssign for Veloc {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}
impl Sub for Veloc {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Veloc {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl SubAssign for Veloc {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl Rem for Veloc {
    type Output = Self;
    fn rem(self, other: Self) -> Self {
        Veloc {
            x: self.x % other.x,
            y: self.y % other.y,
        }
    }
}
impl RemAssign for Veloc {
    fn rem_assign(&mut self, other: Self) {
        *self = *self % other;
    }
}
impl Div<Veloc> for Veloc {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        if other.x == 0.0 || other.y == 0.0 {
            Veloc { x: 0.0, y: 0.0 }
        } else {
            Veloc {
                x: self.x / other.x,
                y: self.y / other.y,
            }
        }
    }
}
impl Div<f64> for Veloc {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        self / Veloc { x: other, y: other}
    }
}
impl Mul<Veloc> for Veloc {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Veloc {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}
impl Mul<f64> for Veloc {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        self * Veloc { x: other, y: other}
    }
}
impl DivAssign<Veloc> for Veloc {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}
impl DivAssign<f64> for Veloc {
    fn div_assign(&mut self, other: f64) {
        *self = *self / other;
    }
}
impl MulAssign<Veloc> for Veloc {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}
impl MulAssign<f64> for Veloc {
    fn mul_assign(&mut self, other: f64) {
        *self = *self * other;
    }
}
impl From<Size> for Veloc {
    fn from(size: Size) -> Self {
        Veloc {
            x: f64::from(size.width),
            y: f64::from(size.height),
        }
    }
}
impl From<[f64; 2]> for Veloc {
    fn from(list: [f64; 2]) -> Self {
        Veloc {
            x: list[0],
            y: list[1],
        }
    }
}
