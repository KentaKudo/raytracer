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

    pub fn color(&self, world: &HittableList) -> Color {
        if let Some(rec) = world.hit(self, 0., std::f64::INFINITY) {
            return 0.5 * (rec.normal + Color::new(1., 1., 1.));
        }

        let unit_direction = self.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
