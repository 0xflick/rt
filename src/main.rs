use image::ImageBuffer;
use rand::prelude::*;

use rt::camera::Camera;
use rt::color::Color;
use rt::material::{Dialectric, Lambertian, Metal};
use rt::point::Point;
use rt::scene::{Hit, HitList, Sphere};
use rt::util::render_ray;
use rt::vector::Vector3;

fn main() {
    let nx = 1500;
    let ny = 750;
    let num_samples = 5;

    let mut rng = thread_rng();

    let camera_origin = Point {
        x: 13.0,
        y: 2.0,
        z: 3.0,
    };
    let look_at = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let v_up = Vector3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    let dist_to_focus = 10.0;
    let aperature = 0.1;
    let camera = Camera::new(
        camera_origin,
        look_at,
        v_up,
        30.0,
        f64::from(nx) / f64::from(ny),
        aperature,
        dist_to_focus,
    );

    let world = random_scene();

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
            col = col + render_ray(&ray, world.as_ref(), 0);
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

fn random_scene() -> Box<Hit> {
    let mut rng = thread_rng();
    let mut hitlist = HitList::new();
    hitlist.push(Sphere::new(
        Point {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        1000.0,
        Box::new(Lambertian {
            albedo: Vector3 {
                x: 0.5,
                y: 0.5,
                z: 0.0,
            },
        }),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Point {
                x: f64::from(a) + 0.9 * rng.gen::<f64>(),
                y: 0.2,
                z: f64::from(b) + 0.9 * rng.gen::<f64>(),
            };
            let t = Point {
                x: 4.0,
                y: 0.2,
                z: 0.0,
            };
            if (center - t).length() > 0.9 {
                if choose_mat < 0.8 {
                    hitlist.push(Sphere::new(
                        center,
                        0.2,
                        Box::new(Lambertian {
                            albedo: Vector3 {
                                x: rng.gen::<f64>() * rng.gen::<f64>(),
                                y: rng.gen::<f64>() * rng.gen::<f64>(),
                                z: rng.gen::<f64>() * rng.gen::<f64>(),
                            },
                        }),
                    ));
                } else if choose_mat < 0.95 {
                    hitlist.push(Sphere::new(
                        center,
                        0.2,
                        Box::new(Metal {
                            albedo: Vector3 {
                                x: 0.5 * (1.0 + rng.gen::<f64>()),
                                y: 0.5 * (1.0 + rng.gen::<f64>()),
                                z: 0.5 * (1.0 + rng.gen::<f64>()),
                            },
                            fuzz: 0.5 * rng.gen::<f64>(),
                        }),
                    ));
                } else {
                    hitlist.push(Sphere::new(
                        center,
                        0.2,
                        Box::new(Dialectric { ref_idx: 1.5 }),
                    ));
                }
            }
        }
    }

    hitlist.push(Sphere::new(
        Point {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        1.0,
        Box::new(Dialectric { ref_idx: 1.5 }),
    ));

    hitlist.push(Sphere::new(
        Point {
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        1.0,
        Box::new(Lambertian {
            albedo: Vector3 {
                x: 0.4,
                y: 0.2,
                z: 0.1,
            },
        }),
    ));

    hitlist.push(Sphere::new(
        Point {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        1.0,
        Box::new(Metal {
            albedo: Vector3 {
                x: 0.7,
                y: 0.6,
                z: 0.5,
            },
            fuzz: 0.0,
        }),
    ));

    Box::new(hitlist)
}
