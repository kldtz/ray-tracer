use std::io::{BufWriter, stdout, Write};
use rand::prelude::*;

use crate::camera::Camera;
use crate::color::{BLACK, get_color};
use crate::hits::{Hittable, Hittables, Sphere};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

mod vec3;
mod utils;
mod color;
mod ray;
mod material;
mod hits;
mod camera;

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

fn random_scene(rng: &mut ThreadRng) -> Hittables<Sphere> {
    let mut hittables: Vec<Sphere> = Vec::new();
    let ground_material = Material::Lambertian { albedo: Vec3::new(0.5, 0.5, 0.5) };
    hittables.push(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material,
    });

    // Small spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere = if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Vec3::new(
                        rng.gen::<f64>() * rng.gen::<f64>(),
                        rng.gen::<f64>() * rng.gen::<f64>(),
                        rng.gen::<f64>() * rng.gen::<f64>(),
                    );
                    let material = Material::Lambertian { albedo };
                    Sphere { center, radius: 0.2, material }
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Vec3::new(
                        0.5 * (1.0 + rng.gen::<f64>()),
                        0.5 * (1.0 + rng.gen::<f64>()),
                        0.5 * (1.0 + rng.gen::<f64>()),
                    );
                    let material = Material::Metal { albedo, fuzz: 0.5 * rng.gen::<f64>() };
                    Sphere { center, radius: 0.2, material }
                } else {
                    // Glass
                    Sphere { center, radius: 0.2, material: Material::Dielectric { refractive_index: 1.5 } }
                };
                hittables.push(sphere);
            }
        }
    }

    // Large spheres
    // Glass
    hittables.push(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Dielectric {refractive_index: 1.5},
    });
    // Diffuse
    hittables.push(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Lambertian {albedo: Vec3::new(0.4, 0.2, 0.1)},
    });
    // Metal
    hittables.push(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Metal {albedo: Vec3::new(0.7, 0.6, 0.5), fuzz: 0.0},
    });

    Hittables { hittables }
}

fn main() -> std::io::Result<()> {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as i64;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let mut rng = rand::thread_rng();

    // World
    let world = random_scene(&mut rng);

    // Camera
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
    );

    // Render
    eprintln!("Rendering {}x{} image", image_width, image_height);
    let mut buf = BufWriter::with_capacity(100 * 1000,stdout());
    writeln!(buf, "P3\n{} {}\n255", image_width, image_height)?;

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen_range(0.0..1.0)) / (image_width as f64 - 1.0);
                let v = (j as f64 + rng.gen_range(0.0..1.0)) / (image_height as f64 - 1.0);
                // Vector from origin to pixel
                let ray = camera.get_ray(u, v, &mut rng);
                pixel_color = pixel_color + ray_color(ray, &world, &mut rng, max_depth);
            }
            writeln!(buf, "{}", get_color(pixel_color, samples_per_pixel))?;
        }
    }
    buf.flush()?;
    eprintln!("Done");
    Ok(())
}
