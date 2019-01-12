use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl Neg for Color {
    type Output = Color;

    fn neg(self) -> Color {
        Color {
            r: -self.r,
            g: -self.g,
            b: -self.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Color {
        Color {
            r: other * self.r,
            g: other * self.g,
            b: other * self.b,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            r: self * other.r,
            g: self * other.g,
            b: self * other.b,
        }
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, other: f64) -> Color {
        Color {
            r: self.r / other,
            g: self.g / other,
            b: self.b / other,
        }
    }
}
