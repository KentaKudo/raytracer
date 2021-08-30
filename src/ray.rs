use crate::vec3::{Color, Point3, Vec3};

pub struct Ray {
    org: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(org: Point3, dir: Vec3) -> Self {
        Self { org, dir }
    }

    fn origin(self) -> Point3 {
        self.org
    }

    fn direction(self) -> Vec3 {
        self.dir
    }

    fn at(self, t: f64) -> Point3 {
        self.org + t * self.dir
    }

    pub fn color(self) -> Color {
        let unit_direction = self.dir.unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
