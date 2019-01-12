use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::point::Point;

#[derive(PartialEq, Copy, Clone, Debug, Default)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn zero() -> Self {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn norm(&self) -> f64 {
        self.dot(self)
    }
    pub fn length(&self) -> f64 {
        self.norm().sqrt()
    }

    pub fn normalize(&self) -> Vector3 {
        Vector3 {
            x: self.x / self.length(),
            y: self.y / self.length(),
            z: self.z / self.length(),
        }
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f64) -> Vector3 {
        Vector3 {
            x: other * self.x,
            y: other * self.y,
            z: other * self.z,
        }
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl From<Point> for Vector3 {
    fn from(pt: Point) -> Self {
        Vector3 {
            x: pt.x,
            y: pt.y,
            z: pt.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zeros() {
        let zeros: Vector3 = Vector3::zero();

        assert_eq!(zeros.x, 0.0);
        assert_eq!(zeros.y, 0.0);
        assert_eq!(zeros.z, 0.0);
    }

    #[test]
    fn test_squared_length() {
        let vec = Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        assert_eq!(vec.norm(), 3.0);
    }

    #[test]
    fn test_length() {
        let vec = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 2.0,
        };

        assert_eq!(vec.length(), 3.0);
    }

    #[test]
    fn test_ops() {
        let vec1 = Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        let vec2 = Vector3 {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };

        assert_eq!(
            vec1 + vec2,
            Vector3 {
                x: 3.0,
                y: 3.0,
                z: 3.0,
            }
        );
        assert_eq!(
            vec2 - vec1,
            Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }
        );
        assert_eq!(
            -vec1,
            Vector3 {
                x: -1.0,
                y: -1.0,
                z: -1.0,
            }
        );
    }

    #[test]
    fn test_scalar_mul() {
        let vec = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        assert_eq!(
            2.0 * vec,
            Vector3 {
                x: 2.0,
                y: 4.0,
                z: 6.0,
            }
        );
    }

    #[test]
    fn test_scalar_div() {
        let vec = Vector3 {
            x: 10.0,
            y: 20.0,
            z: 30.0,
        };

        assert_eq!(
            vec / 10.0,
            Vector3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            }
        );
    }
}
