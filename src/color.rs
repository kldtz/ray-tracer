use crate::utils::clamp;
use crate::Vec3;

const MIN_PERC: f64 = 0.0;
const MAX_PERC: f64 = 0.999;
pub const BLACK: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
pub const WHITE: Vec3 = Vec3 { x: 1.0, y: 1.0, z: 1.0 };

pub fn get_color(pixel_color: Vec3, samples_per_pixel: i64) -> String {
    // Divide color by number of samples
    let scale = 1.0 / samples_per_pixel as f64;
    let pixel_color = pixel_color * scale;

    // Concatenate translated [0, 255] value of each color component
    let r = 256.0 * clamp(pixel_color.x.sqrt(), MIN_PERC, MAX_PERC);
    let g = 256.0 * clamp(pixel_color.y.sqrt(), MIN_PERC, MAX_PERC);
    let b = 256.0 * clamp(pixel_color.z.sqrt(), MIN_PERC, MAX_PERC);
    format!("{} {} {}", r as i64, g as i64, b as i64)
}