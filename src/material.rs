use crate::ray::Ray;
use crate::scene::HitRecord;
use crate::util::random_in_unit_sphere;
use crate::vector::Vector3;
use rand::prelude::*;

pub struct Scatter {
    pub scattered: Ray,
    pub attenuation: Vector3,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<Scatter>;
}

pub struct Lambertian {
    pub albedo: Vector3,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let t = rec.p + rec.normal + random_in_unit_sphere();
        Some(Scatter {
            scattered: Ray {
                origin: rec.p,
                direction: t - rec.p,
            },
            attenuation: self.albedo,
        })
    }
}

fn reflect(v: Vector3, n: Vector3) -> Vector3 {
    v - 2.0 * v.dot(&n) * n
}

pub struct Metal {
    pub albedo: Vector3,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let reflected = reflect(ray.direction.normalize(), rec.normal);
        let scattered = Ray {
            origin: rec.p,
            direction: reflected
                + Vector3::from(self.fuzz * random_in_unit_sphere()),
        };
        if scattered.direction.dot(&rec.normal) > 0.0 {
            Some(Scatter {
                scattered,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}

fn refract(v: Vector3, n: Vector3, ni_over_nt: f64) -> Option<Vector3> {
    let v = v.normalize();
    let dt = v.dot(&n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (v - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub struct Dialectric {
    pub ref_idx: f64,
}

impl Material for Dialectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let (outward_normal, ni_over_nt, cosine) =
            if ray.direction.dot(&rec.normal) > 0.0 {
                (
                    -rec.normal,
                    self.ref_idx,
                    self.ref_idx * ray.direction.dot(&rec.normal)
                        / ray.direction.length(),
                )
            } else {
                (
                    rec.normal,
                    1.0 / self.ref_idx,
                    -ray.direction.dot(&rec.normal) / ray.direction.length(),
                )
            };

        let reflect_prob = schlick(cosine, self.ref_idx);
        let scattered = match (
            refract(ray.direction, outward_normal, ni_over_nt),
            rand::thread_rng().gen::<f64>() < reflect_prob,
        ) {
            (Some(refracted), false) => Ray {
                origin: rec.p,
                direction: refracted,
            },
            (_, _) => Ray {
                origin: rec.p,
                direction: reflect(ray.direction, rec.normal),
            },
        };

        Some(Scatter {
            scattered,
            attenuation: Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        })
    }
}
