use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects = Vec::new()
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object)
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut result = None;
        let mut t_closest = t_max;
        for obj in &self.objects {
            if let Some(rec) = obj.hit(r, t_min, t_closest) {
                t_closest = rec.t;
                result = Some(rec);
            }
        }

        result
    }
}
