use rand::prelude::*;

use crate::camera::Camera;
use crate::color::get_color;
use crate::hits::{Hittable, Hittables, Sphere};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub mod vec3;
pub mod utils;
pub mod color;
pub mod ray;
pub mod hits;
pub mod camera;

pub fn ray_color<T: Hittable>(ray: Ray, world: &Hittables<T>, rng: &mut ThreadRng, depth: i64) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    if let Some(hit) = world.hit(&ray, 0.001, f64::MAX) {
        let target = hit.point + hit.normal + Vec3::random_in_unit_sphere(rng);
        return 0.5 * ray_color(Ray { origin: hit.point, direction: target - hit.point }, world, rng, depth-1);
    }
    let unit_direction = ray.direction.unit_vector();
    // Transform y-value from range [-1, 1] to [0, 1]
    let t = 0.5 * (unit_direction.y + 1.0);
    // Return linear interpolation between white (1, 1, 1) and blue (0.5, 0.7, 1)
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

/// Optionally returns parameter t at which ray intersects sphere (closest hit).
pub fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> Option<f64> {
    let oc = ray.origin - center;
    let a = ray.direction.length_squared();
    let half_b = oc.dot(ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return None;
    }
    Some((-half_b - discriminant.sqrt()) / a)
}

fn first_image() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i64;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let hittables = vec![
        Sphere { center: Vec3::new(0.0, 0.0, -1.0), radius: 0.5 },
        Sphere { center: Vec3::new(0.0, -100.5, -1.0), radius: 100.0 },
    ];
    let world = Hittables { hittables };

    // Camera
    let camera = Camera::new();

    // Render
    eprintln!("Rendering {}x{} image", image_width, image_height);
    println!("P3\n{} {}\n255", image_width, image_height);

    let mut rng = rand::thread_rng();

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen_range(0.0..1.0)) / (image_width as f64 - 1.0);
                let v = (j as f64 + rng.gen_range(0.0..1.0)) / (image_height as f64 - 1.0);
                // Vector from origin to pixel
                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(ray, &world, &mut rng, max_depth);
            }
            println!("{}", get_color(pixel_color, samples_per_pixel));
        }
    }
    eprintln!("Done");
}


fn main() {
    first_image();
}
