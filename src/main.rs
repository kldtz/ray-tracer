use crate::ray::Ray;
use crate::vec3::Vec3;

pub mod vec3;
pub mod ray;

pub fn ray_color(ray: Ray) -> Vec3 {
    let t =  hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, &ray);
    if let Some(t) = t {
        let normal = ray.at(t) - Vec3::new(0.0, 0.0, -1.0);
        return 0.5 * Vec3::new(normal.x+1.0, normal.y + 1.0, normal.z + 1.0);
    }
    let unit_direction = ray.direction.unit_vector();
    // Transform y-value from range [-1, 1] to [0, 1]
    let t = 0.5 * (unit_direction.y + 1.0);
    // Return linear interpolation between white (1, 1, 1) and blue (0.5, 0.7, 1)
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

/// Optionally returns parameter t at which ray intersects sphere.
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
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i64;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    // Position of eye/camera
    let origin = Vec3::new(0.0, 0.0, 0.0);
    // Left: negative x, right: positive x
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    // Up: positive y, down: negative y
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    // Lower left corner of the viewport, the viewport is at negative z-value focal_length
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width as f64 - 1.0);
            let v = j as f64 / (image_height as f64 - 1.0);
            // Vector from origin to pixel
            let direction = lower_left_corner + u * horizontal + v * vertical - origin;
            let ray = Ray { origin, direction };
            let ray_color = ray_color(ray);
            println!("{}", ray_color.to_color());
        }
    }
    eprintln!("Done");
}


fn main() {
    first_image();
}
