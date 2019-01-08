extern crate image;

use image::ImageBuffer;

mod vec3;
use vec3::Vec3;

fn main() {
    let nx = 200;
    let ny = 100;

    let img = ImageBuffer::from_fn(nx, ny, |x, y| {
        let col = Vec3::new(x as f64 / nx as f64, (ny - y) as f64 / ny as f64, 0.2);
        image::Rgb([
            (col[0] * 255.99) as u8,
            (col[1] * 255.99) as u8,
            (col[2] * 255.99) as u8,
        ])
    });

    img.save("out.jpg").unwrap();
}
