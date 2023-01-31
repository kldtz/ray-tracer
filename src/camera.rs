use crate::{Ray, Vec3};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;

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
        Camera { origin, lower_left_corner, horizontal, vertical }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let direction = self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin;
        Ray { origin: self.origin, direction }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}