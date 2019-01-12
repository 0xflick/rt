use crate::ray::Ray;
use crate::scene::HitRecord;
use crate::util::random_in_unit_sphere;
use crate::vector::Vector3;

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
