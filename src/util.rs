use rand::prelude::*;

use crate::color::Color;
use crate::point::Point;
use crate::ray::Ray;
use crate::scene::Hit;
use crate::vector::Vector3;

pub fn render_ray<T: Hit + ?Sized>(
    ray: &Ray,
    world: &T,
    depth: usize,
) -> Color {
    if let Some(rec) = world.hit(ray, 0.001, std::f64::INFINITY) {
        match (depth < 50, rec.material.scatter(ray, &rec)) {
            (true, Some(s)) => {
                let col = render_ray(&s.scattered, world, depth + 1);
                Color {
                    r: col.r * s.attenuation.x,
                    g: col.g * s.attenuation.y,
                    b: col.b * s.attenuation.z,
                }
            }
            (_, _) => Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            },
        }
    } else {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * unit_direction.y + 1.0;
        let white = Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        };
        let light_blue = Color {
            r: 0.5,
            g: 0.7,
            b: 1.0,
        };

        (1.0 - t) * white + t * light_blue
    }
}

pub fn random_in_unit_sphere() -> Point {
    let mut rng = thread_rng();
    loop {
        let p =
            2.0 * Vector3 {
                x: rng.gen(),
                y: rng.gen(),
                z: rng.gen(),
            } - Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
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
