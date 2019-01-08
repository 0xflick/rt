extern crate image;

use image::ImageBuffer;

fn main() {
    let nx = 200;
    let ny = 100;

    let img = ImageBuffer::from_fn(nx, ny, |x, y| {
        let ir = ((x as f64 / nx as f64) * 255.99) as u8;
        let ig = (((ny - y) as f64 / ny as f64) * 255.99) as u8;
        let ib = (0.2 * 255.99) as u8;
        image::Rgb([ir, ig, ib])
    });

    img.save("out.jpg").unwrap();
}
