use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    org: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,            // "view up"
        vfov_in_degrees: f64, // vertical field-of-view
        aspect_ratio: f64,
    ) -> Self {
        let theta = vfov_in_degrees.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let org = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        Self {
            org,
            lower_left_corner: org - horizontal / 2. - vertical / 2. - w,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.org,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.org,
        )
    }
}
