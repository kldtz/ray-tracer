use ray_tracing::render;
use ray_tracing::configs::random_spheres;

fn main() -> std::io::Result<()> {
    let mut rng = rand::thread_rng();
    let conf = random_spheres(&mut rng);
    render(conf, &mut rng)
}
