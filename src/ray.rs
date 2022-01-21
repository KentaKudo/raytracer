use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::vec3::{Color, Point3, Vec3};

pub struct Ray {
    org: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(org: Point3, dir: Vec3) -> Self {
        Self { org, dir }
    }

    pub fn origin(&self) -> Point3 {
        self.org
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.org + t * self.dir
    }

    pub fn color(&self, world: &HittableList, depth: i64) -> Color {
        if depth <= 0 {
            return Color::default();
        }

        if let Some(rec) = world.hit(self, 0.001, std::f64::INFINITY) {
            // Pick one from different diffuse approaches.

            // 1. True Lambertian Reflection
            let target = rec.p + rec.normal + Vec3::random_unit_vector();

            // 2. Simple Diffuse
            // let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();

            // 3. Uniform scatter for all angles away from hit point
            // let target = rec.p + Vec3::random_in_hemisphere(rec.normal);

            return 0.5 * Ray::new(rec.p, target - rec.p).color(world, depth - 1);
        }

        let unit_direction = self.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
