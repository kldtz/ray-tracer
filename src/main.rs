use rand::prelude::*;

use crate::camera::Camera;
use crate::color::{BLACK, get_color};
use crate::hits::{Hittable, Hittables, Sphere};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub mod vec3;
pub mod utils;
pub mod color;
pub mod ray;
mod material;
pub mod hits;
pub mod camera;

pub fn ray_color<T: Hittable>(ray: Ray, world: &Hittables<T>, rng: &mut ThreadRng, depth: i64) -> Vec3 {
    if depth <= 0 {
        return BLACK;
    }
    if let Some(hit) = world.hit(&ray, 0.001, f64::MAX) {
        if let Some(scattered) = hit.material.scatter(&ray, &hit, rng) {
            return hit.material.attenuate(ray_color(scattered, world, rng, depth - 1));
        }
        return BLACK;
    }
    let unit_direction = ray.direction.unit_vector();
    // Transform y-value from range [-1, 1] to [0, 1]
    let t = 0.5 * (unit_direction.y + 1.0);
    // Return linear interpolation between white (1, 1, 1) and blue (0.5, 0.7, 1)
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i64;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // Materials
    let ground = Material::Lambertian { albedo: Vec3::new(0.8, 0.8, 0.0) };
    let center = Material::Lambertian { albedo: Vec3::new(0.7, 0.3, 0.3) };
    let left_metal = Material::Metal { albedo: Vec3::new(0.8, 0.8, 0.8) };
    let right_metal = Material::Metal { albedo: Vec3::new(0.8, 0.6, 0.2) };

    // World
    let hittables = vec![
        Sphere { center: Vec3::new(0.0, 0.0, -1.0), radius: 0.5, material: &center },
        Sphere { center: Vec3::new(0.0, -100.5, -1.0), radius: 100.0, material: &ground },
        Sphere { center: Vec3::new(-1.0, 0.0, -1.0), radius: 0.5, material: &left_metal },
        Sphere { center: Vec3::new(1.0, 0.0, -1.0), radius: 0.5, material: &right_metal },
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
