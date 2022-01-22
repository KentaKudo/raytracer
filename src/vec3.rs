use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

#[derive(Copy, Clone)]
pub struct Vec3(f64, f64, f64);

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn default() -> Self {
        Self(0., 0., 0.)
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn random(min: f64, max: f64) -> Self {
        let mut rng = match SmallRng::from_rng(rand::thread_rng()) {
            Ok(rng) => rng,
            _ => return Self::default(),
        };

        Self(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random(-1., 1.);
            if p.length_squared() < 1. {
                break p;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_hemisphere(normal: Self) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn x(self) -> f64 {
        self.0
    }

    pub fn y(self) -> f64 {
        self.1
    }

    pub fn z(self) -> f64 {
        self.2
    }

    fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        self.x() < s && self.y() < s && self.z() < s
    }
}

use std::ops;

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.x(), -self.y(), -self.z())
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        rhs * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        (1.0 / rhs) * self
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        );
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self(self.x() * other, self.y() * other, self.z() * other)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self = Self(self.x() / other, self.y() / other, self.z() / other)
    }
}

impl Vec3 {
    pub fn dot(self, other: Self) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    fn cross(self, other: Self) -> Self {
        Self(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    pub fn unit_vector(self) -> Self {
        self / self.length()
    }

    pub fn reflect(self, n: Self) -> Self {
        self - 2. * self.dot(n) * n
    }
}

use std::fmt;

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

pub fn print_color(pixel_color: Color, samples_per_pixel: i64) {
    let Vec3(r, g, b) = pixel_color / samples_per_pixel as f64;
    let (r, g, b) = (r.sqrt(), g.sqrt(), b.sqrt());

    println!(
        "{} {} {}",
        (256. * r.clamp(0., 0.999)) as i32,
        (256. * g.clamp(0., 0.999)) as i32,
        (256. * b.clamp(0., 0.999)) as i32,
    )
}
