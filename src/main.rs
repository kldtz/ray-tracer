use ray_tracing::configs::moving_random_spheres;
use ray_tracing::render;

fn main() -> std::io::Result<()> {
    let mut rng = rand::thread_rng();
    let conf = moving_random_spheres(&mut rng);
    render(conf, &mut rng)
}
