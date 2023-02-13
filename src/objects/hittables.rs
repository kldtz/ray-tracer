use crate::objects::{Hit, Hittable};
use crate::Ray;

pub struct Hittables {
    pub hittables: Vec<Box<dyn Hittable>>,
}

impl Hittable for Hittables {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut hit: Option<Hit> = None;
        let mut closest = t_max;

        for hittable in self.hittables.iter() {
            if let Some(h) = hittable.hit(ray, t_min, closest) {
                closest = h.t;
                hit = Some(h);
            }
        }
        hit
    }
}