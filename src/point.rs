use std::ops::{Add, Mul, Neg, Sub};

use crate::vector::Vector3;

#[derive(PartialEq, Copy, Clone, Debug, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn origin() -> Self {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<Vector3> for Point {
    type Output = Point;

    fn add(self, other: Vector3) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point {
    type Output = Vector3;

    fn sub(self, other: Point) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, other: f64) -> Point {
        Point {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        other * self
    }
}

impl Sub<Vector3> for Point {
    type Output = Point;

    fn sub(self, other: Vector3) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_origin() {
        let pt = Point::origin();

        assert_eq!(
            pt,
            Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        )
    }

    #[test]
    fn test_ops() {
        let pt1 = Point::origin();
        let pt2 = Point {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        assert_eq!(
            pt1 + pt2,
            Point {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }
        );

        assert_eq!(
            pt2 - pt1,
            Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }
        );

        assert_eq!(
            pt1 - pt2,
            Vector3 {
                x: -1.0,
                y: -1.0,
                z: -1.0,
            }
        );

        assert_eq!(
            -pt2,
            Point {
                x: -1.0,
                y: -1.0,
                z: -1.0,
            }
        )
    }
}
