use crate::Vec3;

pub struct Ray<'a> {
    pub origin: &'a Vec3,
    pub direction: &'a Vec3,
}

impl Ray<'_> {
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + &(self.direction * t)
    }

    pub fn color(&self) -> Vec3 {
        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}
