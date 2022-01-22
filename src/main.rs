mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod vec3;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use camera::Camera;
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Metal};
use sphere::Sphere;
use vec3::{print_color, Color, Point3, Vec3};

fn main() -> Result<(), rand::Error> {
    let mut rng = SmallRng::from_rng(rand::thread_rng())?;

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList::new();

    let material_ground = Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.)));
    let material_center = Box::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Box::new(Dielectric::new(1.5));
    let material_right = Box::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Box::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        material_center,
    )));

    // Hollow glass sphere
    world.add(Box::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        material_left.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        -0.45,
        material_left,
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        material_right,
    )));

    // Camera
    let cam = Camera::new(
        Point3::new(-2., 2., 1.),
        Point3::new(0., 0., -1.),
        Vec3::new(0., 1., 0.),
        20.,
        aspect_ratio,
    );

    // Render
    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {j}");

        for i in 0..image_width {
            let mut pixel_color = Color::default();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen_range(0.0..1.0)) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen_range(0.0..1.0)) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += r.color(&world, max_depth);
            }
            print_color(pixel_color, samples_per_pixel);
        }
    }

    eprintln!("Done.");

    Ok(())
}
