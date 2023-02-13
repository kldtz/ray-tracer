use rand::prelude::ThreadRng;
use rand::Rng;
use crate::{Camera, Material, Vec3};
use crate::configs::ImageConfig;
use crate::objects::{Hittable, Hittables, Sphere};

fn random_scene(rng: &mut ThreadRng) -> Hittables {
    let mut hittables: Vec<Box<dyn Hittable>> = Vec::new();
    let ground_material = Material::Lambertian { albedo: Vec3::new(0.5, 0.5, 0.5) };
    hittables.push(Box::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material,
    }));

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
                hittables.push(Box::new(sphere));
            }
        }
    }

    // Large spheres
    // Glass
    hittables.push(Box::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Dielectric { refractive_index: 1.5 },
    }));
    // Diffuse
    hittables.push(Box::new(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Lambertian { albedo: Vec3::new(0.4, 0.2, 0.1) },
    }));
    // Metal
    hittables.push(Box::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Metal { albedo: Vec3::new(0.7, 0.6, 0.5), fuzz: 0.0 },
    }));

    Hittables { hittables }
}

pub fn random_spheres(rng: &mut ThreadRng) -> ImageConfig {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as i64;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // World
    let world = random_scene(rng);

    // Camera
    let camera = Camera::new(
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
    );
    ImageConfig {
        aspect_ratio,
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
        world,
        camera,
    }
}