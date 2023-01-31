use crate::ray::Ray;
use crate::vec3::Vec3;

pub mod vec3;
pub mod ray;

fn first_image() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i64;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = &origin - &(&horizontal / 2.0) - &vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);


    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width as f64 - 1.0);
            let v = j as f64 / (image_height as f64 - 1.0);
            let direction = &(&lower_left_corner + &(u * &horizontal) + v * &vertical) - &origin;
            let ray = Ray { origin: &origin, direction: &direction };
            let ray_color = ray.color();
            println!("{}", ray_color.to_color());
        }
    }
    eprintln!("Done");
}


fn main() {
    first_image();
}
