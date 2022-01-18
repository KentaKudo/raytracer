use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    p: Point3,
    pub normal: Vec3,
    pub t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, t: f64) -> Self {
        Self {
            p,
            normal: Vec3::default(),
            t,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
