use crate::{Material, Ray, Vec3};
use crate::objects::{Hit, Hittable};

pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub material: Material,
}

impl MovingSphere {
    /// Returns center position of sphere at given time via linear interpolation (assuming constant speed).
    pub fn center(&self, time: f64) -> Vec3 {
        self.center0 + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = ray.origin - self.center(ray.time);
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find nearest root that lies in acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = ray.at(root);
        // Center position of sphere depends on time
        let outward_normal = (point - self.center(ray.time)) / self.radius;
        let t = root;
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };

        Some(Hit { point, normal, t, front_face, material: &self.material })
    }
}