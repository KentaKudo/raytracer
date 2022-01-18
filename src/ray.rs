use crate::vec3::{Color, Point3, Vec3};

pub struct Ray {
    org: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(org: Point3, dir: Vec3) -> Self {
        Self { org, dir }
    }

    fn origin(&self) -> Point3 {
        self.org
    }

    fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.org + t * self.dir
    }

    pub fn color(&self) -> Color {
        if self.hit_sphere(Point3::new(0., 0., -1.), 0.5) {
            return Color::new(1.0, 0., 0.);
        }

        let unit_direction = self.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }

    fn hit_sphere(&self, center: Point3, radius: f64) -> bool {
        let oc = self.origin() - center;
        let a = self.direction().dot(self.direction());
        let b = 2.0 * oc.dot(self.direction());
        let c = oc.dot(oc) - radius * radius;
        let discriminant = b * b - 4. * a * c;
        discriminant > 0.
    }
}
