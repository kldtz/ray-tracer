use rand::rngs::ThreadRng;

use crate::{Ray, Vec3};
use crate::hits::Hit;

pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f64 },
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut ThreadRng) -> Option<Ray> {
        match *self {
            Material::Lambertian { albedo: _ } => {
                let mut scatter_direction = hit.normal + Vec3::random_in_unit_sphere(rng).unit_vector();
                if scatter_direction.near_zero() {
                    scatter_direction = hit.normal;
                }
                Some(Ray { origin: hit.point, direction: scatter_direction })
            }
            Material::Metal { albedo: _, fuzz } => {
                let reflected = reflect(ray.direction.unit_vector(), hit.normal);
                if reflected.dot(hit.normal) > 0.0 {
                    Some(Ray {
                        origin: hit.point,
                        direction: reflected + fuzz * Vec3::random_in_unit_sphere(rng),
                    })
                } else {
                    None
                }
            }
        }
    }

    pub fn attenuate(&self, color: Vec3) -> Vec3 {
        match *self {
            Material::Lambertian { albedo } | Material::Metal { albedo, .. } => albedo * color,
        }
    }
}

fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    v - 2.0 * v.dot(normal) * normal
}
