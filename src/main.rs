extern crate image;
extern crate rt;

use image::ImageBuffer;

use rt::color::Color;

fn main() {
    let nx = 1280;
    let ny = 1024;

    let img = ImageBuffer::from_fn(nx, ny, |x, y| {
        let col: Color<f64> = Color {
            r: f64::from(x) / f64::from(nx),
            g: f64::from(ny - y) / f64::from(ny),
            b: 0.2,
        };

        image::Rgb([
            (col.r * 255.00) as u8,
            (col.g * 255.00) as u8,
            (col.b * 255.00) as u8,
        ])
    });

    img.save("out.jpg").unwrap();
}
