use crate::point::Point;
use crate::vector::Vector3;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point {
        self.origin + t * self.direction
    }
}
