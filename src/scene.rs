use crate::material::Material;
use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector3;

pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Point,
    pub normal: Vector3,
    pub material: &'a Material,
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Box<Material>,
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let direction = ray.origin - self.center;
        let a = ray.direction.norm();
        let b = direction.dot(&ray.direction);
        let c = direction.norm() - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let t0 = (-b - discriminant.sqrt()) / a;
            let t1 = (-b + discriminant.sqrt()) / a;

            if t0 > t_max || t1 < t_min {
                return None;
            };

            let t_hit = if t0 < t_min { t1 } else { t0 };

            let p = ray.at(t_hit);
            Some(HitRecord {
                t: t_hit,
                p,
                normal: ((p - self.center) * self.radius.signum()).normalize(),
                material: self.material.as_ref(),
            })
        } else {
            None
        }
    }
}

#[derive(Default)]
pub struct HitList<'a> {
    data: Vec<Box<Hit + 'a>>,
}

impl<'a> HitList<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push<T: Hit + 'a>(&mut self, hitable: T) {
        self.data.push(Box::new(hitable));
    }
}

impl<'a> Hit for HitList<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest: Option<HitRecord> = None;
        for hitable in &self.data {
            let t = closest.as_ref().map_or(t_max, |h| h.t);
            closest = hitable.hit(ray, t_min, t).or(closest);
        }
        closest
    }
}
