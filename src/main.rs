use image::ImageBuffer;
use rand::prelude::*;

use rt::camera::Camera;
use rt::color::Color;
use rt::material::{Lambertian, Metal};
use rt::point::Point;
use rt::scene::{HitList, Sphere};
use rt::util::render_ray;
use rt::vector::Vector3;

fn main() {
    let nx = 1500;
    let ny = 750;
    let num_samples = 1000;

    let mut rng = thread_rng();
    let camera = Camera::new();
    let mut world = HitList::new();
    world.push(Sphere {
        center: Point {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        material: Box::new(Lambertian {
            albedo: Vector3 {
                x: 0.8,
                y: 0.3,
                z: 0.3,
            },
        }),
    });
    world.push(Sphere {
        center: Point {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
        material: Box::new(Lambertian {
            albedo: Vector3 {
                x: 0.8,
                y: 0.8,
                z: 0.0,
            },
        }),
    });
    world.push(Sphere {
        center: Point {
            x: 1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        material: Box::new(Metal {
            albedo: Vector3 {
                x: 0.8,
                y: 0.6,
                z: 0.2,
            },
            fuzz: 1.0,
        }),
    });
    world.push(Sphere {
        center: Point {
            x: -1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        material: Box::new(Metal {
            albedo: Vector3 {
                x: 0.8,
                y: 0.8,
                z: 0.8,
            },
            fuzz: 0.3,
        }),
    });

    let img = ImageBuffer::from_fn(nx, ny, |x, y| {
        let mut col = Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        };
        for _ in 0..num_samples {
            let u = (f64::from(x) + rng.gen::<f64>()) / f64::from(nx);
            let v = (f64::from(y) + rng.gen::<f64>()) / f64::from(ny);

            let ray = camera.get_ray(u, v);
            col = col + render_ray(&ray, &world, 0);
        }

        col = col / f64::from(num_samples);
        image::Rgb([
            (col.r.sqrt() * 255.0) as u8,
            (col.g.sqrt() * 255.0) as u8,
            (col.b.sqrt() * 255.0) as u8,
        ])
    });

    image::imageops::flip_vertical(&img)
        .save("out1.png")
        .unwrap();
}
