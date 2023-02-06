use rand::Rng;
use rand::rngs::ThreadRng;

use crate::{Ray, Vec3};
use crate::hits::Hit;

pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f64 },
    Dielectric { refractive_index: f64 },
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
            Material::Dielectric { refractive_index } => {
                let refraction_ratio = if hit.front_face { 1.0 / refractive_index } else { refractive_index };

                let unit_direction = ray.direction.unit_vector();
                let cos_theta = f64::min(-unit_direction.dot(hit.normal), 1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                let direction = if cannot_refract
                    || reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.0) {
                    reflect(unit_direction, hit.normal)
                } else {
                    refract(unit_direction, hit.normal, refraction_ratio)
                };
                Some(Ray { origin: hit.point, direction })
            }
        }
    }

    pub fn attenuate(&self, color: Vec3) -> Vec3 {
        match *self {
            Material::Lambertian { albedo } | Material::Metal { albedo, .. } => albedo * color,
            Material::Dielectric { .. } => color,
        }
    }
}

fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    v - 2.0 * v.dot(normal) * normal
}

fn refract(uv: Vec3, normal: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (-uv).dot(normal).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * normal);
    let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * normal;
    r_out_perp + r_out_parallel
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}