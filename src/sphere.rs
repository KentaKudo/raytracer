use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Point3;

pub struct Sphere {
    cen: Point3,
    r: f64,
    mat: Box<dyn Material>,
}

impl Sphere {
    pub fn new(cen: Point3, r: f64, mat: Box<dyn Material>) -> Self {
        Self { cen, r, mat }
    }

    fn center(&self) -> Point3 {
        self.cen
    }

    fn radius(&self) -> f64 {
        self.r
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center();
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius() * self.radius();

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let root = if t_min <= ((-half_b - sqrtd) / a) && ((-half_b - sqrtd) / a) <= t_max {
            (-half_b - sqrtd) / a
        } else if t_min <= ((-half_b + sqrtd) / a) && ((-half_b + sqrtd) / a) <= t_max {
            (-half_b + sqrtd) / a
        } else {
            return None;
        };

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center()) / self.radius();
        let mut rec = HitRecord::new(p, t, self.mat.as_ref());
        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }
}
