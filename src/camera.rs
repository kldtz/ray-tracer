use rand::rngs::ThreadRng;

use crate::{Ray, Vec3};
use crate::utils::degrees_to_radians;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        // Position of eye/camera
        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;

        Camera { origin, lower_left_corner, horizontal, vertical, u, v, w, lens_radius }
    }

    pub fn get_ray(&self, s: f64, t: f64, rng: &mut ThreadRng) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk(rng);
        let offset = self.u * rd.x + self.v * rd.y;

        let origin = self.origin + offset;
        let direction = self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset;
        Ray { origin, direction }
    }
}