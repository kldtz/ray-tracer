use crate::vec3::Vec3;

pub mod vec3;

fn first_image() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let color = Vec3::new(
                i as f64 / (image_width as f64 - 1.0),
                j as f64 / (image_height as f64 - 1.0),
                0.25,
            );
            println!("{}", color.to_color());
        }
    }
    eprintln!("Done");
}


fn main() {
    first_image();
}
