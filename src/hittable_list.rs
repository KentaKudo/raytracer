use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::hittable::{HitRecord, Hittable};
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Color, Point3};

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

impl HittableList {
    pub fn random_scene() -> Result<Self, rand::Error> {
        let mut rng = SmallRng::from_rng(rand::thread_rng())?;

        let mut world = Self::new();

        let ground_material = Box::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        world.add(Box::new(Sphere::new(
            Point3::new(0., -1000., 0.),
            1000.,
            ground_material,
        )));

        for a in -11..11 {
            for b in -11..11 {
                let center = Point3::new(
                    a as f64 + 0.9 * rng.gen_range(0.0..1.0),
                    0.2,
                    b as f64 + 0.9 * rng.gen_range(0.0..1.0),
                );

                if (center - Point3::new(4., 0.2, 0.)).length() <= 0.9 {
                    continue;
                }

                let mat: Box<dyn Material> = match rng.gen_range(0.0..1.0) {
                    f if 0.0 <= f && f < 0.8 => {
                        // diffuse
                        let albedo = Color::random(0., 1.) * Color::random(0., 1.);
                        Box::new(Lambertian::new(albedo))
                    }
                    f if 0.8 <= f && f < 0.95 => {
                        // metal
                        let albedo = Color::random(0.5, 1.);
                        let fuzz = rng.gen_range(0.0..0.5);
                        Box::new(Metal::new(albedo, fuzz))
                    }
                    _ => Box::new(Dielectric::new(1.5)), // glass
                };

                world.add(Box::new(Sphere::new(center, 0.2, mat)));
            }
        }

        world.add(Box::new(Sphere::new(
            Point3::new(0., 1., 0.),
            1.,
            Box::new(Dielectric::new(1.5)),
        )));
        world.add(Box::new(Sphere::new(
            Point3::new(-4., 1., 0.),
            1.,
            Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))),
        )));
        world.add(Box::new(Sphere::new(
            Point3::new(4., 1., 0.),
            1.,
            Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
        )));

        Ok(world)
    }
}
