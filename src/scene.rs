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
    radius2: f64,
}

fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discr = b * b - 4.0 * a * c;
    if discr < 0.0 {
        None
    } else if discr == 0.0 {
        let x = -0.5 * b / a;
        Some((x, x))
    } else {
        let q = if b > 0.0 {
            -0.5 * (b + discr.sqrt())
        } else {
            -0.5 * (b - discr.sqrt())
        };
        let t0 = q / a;
        let t1 = c / q;

        if t1 < t0 {
            Some((t1, t0))
        } else {
            Some((t0, t1))
        }
    }
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Box<Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
            radius2: radius * radius,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let direction = ray.origin - self.center;
        let a = ray.direction.norm();
        let b = 2.0 * direction.dot(&ray.direction);
        let c = direction.norm() - self.radius2;
        if let Some((t0, t1)) = solve_quadratic(a, b, c) {
            if t0 > t_max || t1 < t_min {
                None
            } else {
                let t = if t0 < t_min { t1 } else { t0 };
                let p = ray.at(t);
                Some(HitRecord {
                    t,
                    p,
                    normal: ((p - self.center) * self.radius.signum())
                        .normalize(),
                    material: self.material.as_ref(),
                })
            }
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
        let mut t = t_max;
        for hitable in &self.data {
            if let Some(rec) = hitable.hit(ray, t_min, t) {
                t = rec.t;
                closest = Some(rec);
            }
        }
        closest
    }
}
