use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

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

#[derive(Clone)]
pub struct Dielectric {
    /// Index of refraction
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = ((1. - ref_idx) / (1. + ref_idx)).powi(2);
        r0 + (1. - r0) * (1. - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if rec.front_face {
            1. / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let reflectivity = if let Ok(mut rng) = SmallRng::from_rng(rand::thread_rng()) {
            // Schlick's approximation
            Self::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.0)
        } else {
            false
        };

        let direction = if refraction_ratio * sin_theta > 1.0 || reflectivity {
            // reflection
            unit_direction.reflect(rec.normal)
        } else {
            // refraction
            unit_direction.refract(rec.normal, refraction_ratio)
        };

        Some((Color::new(1., 1., 1.), Ray::new(rec.p, direction)))
    }
}
