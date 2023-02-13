use std::io::{BufWriter, stdout, Write};

use rand::prelude::ThreadRng;
use rand::Rng;

use crate::{Ray, Vec3};
use crate::color::{BLACK, get_color};
use crate::configs::ImageConfig;
use crate::objects::{Hittable, Hittables};

fn ray_color(ray: Ray, world: &Hittables, rng: &mut ThreadRng, depth: i64) -> Vec3 {
    if depth <= 0 {
        BLACK
    } else if let Some(hit) = world.hit(&ray, 0.001, f64::MAX) {
        if let Some(scattered) = hit.material.scatter(&ray, &hit, rng) {
            hit.material.attenuate(ray_color(scattered, world, rng, depth - 1))
        } else {
            BLACK
        }
    } else {
        let unit_direction = ray.direction.unit_vector();
        // Transform y-value from range [-1, 1] to [0, 1]
        let t = 0.5 * (unit_direction.y + 1.0);
        // Return linear interpolation between white (1, 1, 1) and blue (0.5, 0.7, 1)
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

/// Takes [ImageConfig] and renders PPM image to stdout.
pub fn render(conf: ImageConfig, rng: &mut ThreadRng) -> std::io::Result<()> {
    eprintln!("Rendering {}x{} image", conf.image_width, conf.image_height);
    let mut buf = BufWriter::with_capacity(100 * 1000, stdout());
    writeln!(buf, "P3\n{} {}\n255", conf.image_width, conf.image_height)?;

    for j in (0..conf.image_height).rev() {
        for i in 0..conf.image_width {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..conf.samples_per_pixel {
                let u = (i as f64 + rng.gen_range(0.0..1.0)) / (conf.image_width as f64 - 1.0);
                let v = (j as f64 + rng.gen_range(0.0..1.0)) / (conf.image_height as f64 - 1.0);
                // Vector from origin to pixel
                let ray = conf.camera.get_ray(u, v, rng);
                pixel_color = pixel_color + ray_color(ray, &conf.world, rng, conf.max_depth);
            }
            writeln!(buf, "{}", get_color(pixel_color, conf.samples_per_pixel))?;
        }
    }
    buf.flush()?;
    eprintln!("Done");
    Ok(())
}