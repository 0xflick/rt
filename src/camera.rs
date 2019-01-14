use rand::prelude::*;

use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector3;

pub struct Camera {
    origin: Point,
    start: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    u: Vector3,
    v: Vector3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        origin: Point,
        look_at: Point,
        v_up: Vector3,
        vertical_fov: f64,
        aspect_ratio: f64,
        aperature: f64,
        focus_dist: f64,
    ) -> Self {
        let lens_radius = aperature / 2.0;
        let vertical_fov = vertical_fov * (std::f64::consts::PI / 180.00);
        let half_height = (vertical_fov / 2.0).tan();
        let half_width = half_height * aspect_ratio;
        let w = (look_at - origin).normalize();
        let u = w.cross(&v_up).normalize();
        let v = u.cross(&w);
        let start = focus_dist * (-(half_width * u + half_height * v) + w);
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;
        Camera {
            origin,
            start,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let lens_pt = self.lens_radius * random_in_unit_disc();
        let offset = self.u * lens_pt.x + self.v * lens_pt.y;
        Ray {
            origin: self.origin + offset,
            direction: self.start + s * self.horizontal + t * self.vertical
                - offset,
        }
    }
}

fn random_in_unit_disc() -> Point {
    let mut rng = thread_rng();
    loop {
        let p =
            2.0 * Vector3 {
                x: rng.gen(),
                y: rng.gen(),
                z: 0.0,
            } - Vector3 {
                x: 1.0,
                y: 1.0,
                z: 0.0,
            };
        if p.norm() < 1.0 {
            return Point {
                x: p.x,
                y: p.y,
                z: p.z,
            };
        };
    }
}
