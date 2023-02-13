//! Ray tracer following Peter Shirley's books.
pub use camera::Camera;
pub use material::Material;
pub use ray::Ray;
pub use raytracer::render;
pub use vec3::Vec3;

mod vec3;
pub mod utils;
pub mod color;
mod ray;
mod material;
pub mod objects;
mod camera;
pub mod configs;
mod raytracer;

