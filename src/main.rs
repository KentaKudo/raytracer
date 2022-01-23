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
use vec3::{print_color, Color, Point3, Vec3};

fn main() -> Result<(), rand::Error> {
    let mut rng = SmallRng::from_rng(rand::thread_rng())?;

    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let world = HittableList::random_scene()?;

    // Camera
    let look_from = Point3::new(13., 2., 3.);
    let look_at = Point3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        20.,
        aspect_ratio,
        aperture,
        dist_to_focus,
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
