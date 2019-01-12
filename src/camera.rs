use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector3;

pub struct Camera {
    origin: Point,
    start: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            origin: Point::origin(),
            start: Vector3 {
                x: -2.0,
                y: -1.0,
                z: -1.0,
            },
            horizontal: Vector3 {
                x: 4.0,
                y: 0.0,
                z: 0.0,
            },
            vertical: Vector3 {
                x: 0.0,
                y: 2.0,
                z: 0.0,
            },
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.start + u * self.horizontal + v * self.vertical,
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new()
    }
}
