use crate::{Ray, Vec3};
use crate::utils::degrees_to_radians;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom- lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        // Position of eye/camera
        let origin = lookfrom;
        // Left: negative x, right: positive x
        let horizontal = viewport_width * u;
        // Up: positive y, down: negative y
        let vertical = viewport_height * v;
        // Lower left corner of the viewport
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;
        Camera { origin, lower_left_corner, horizontal, vertical }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let direction = self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin;
        Ray { origin: self.origin, direction }
    }
}