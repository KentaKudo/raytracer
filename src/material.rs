use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(a: Color) -> Self {
        Self { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        // Pick one from different diffuse approaches.

        // 1. True Lambertian Reflection
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // 2. Simple Diffuse
        // let mut scatter_direction = rec.normal + Vec3::random_in_unit_sphere();

        // 3. Uniform scatter for all angles away from hit point
        // let mut scatter_direction = rec.normal + Vec3::random_in_hemisphere(rec.normal);

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some((self.albedo, Ray::new(rec.p, scatter_direction)))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(a: Color, f: f64) -> Self {
        Self {
            albedo: a,
            fuzz: f.min(1.),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.direction().unit_vector().reflect(rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        if scattered.direction().dot(rec.normal) > 0. {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
