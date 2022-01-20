mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use rand::Rng;

use camera::Camera;
use hittable_list::HittableList;
use sphere::Sphere;
use vec3::{print_color, Color, Point3};

fn main() {
    let mut rng = rand::thread_rng();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    // Camera
    let cam = Camera::new();

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
                pixel_color += r.color(&world);
            }
            print_color(pixel_color, samples_per_pixel);
        }
    }

    eprintln!("Done.");
}
