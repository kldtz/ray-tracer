pub use random_spheres::*;

use crate::Camera;
use crate::objects::Hittables;

pub struct ImageConfig {
    pub aspect_ratio: f64,
    pub image_width: i64,
    pub image_height: i64,
    pub samples_per_pixel: i64,
    pub max_depth: i64,
    pub world: Hittables,
    pub camera: Camera,
}

mod random_spheres;

