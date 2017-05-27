extern crate image;

use image::*;

fn main() {
    let image = open("image.png").expect("image").to_rgba();

    let x0 = 1250;
    let y0 = 400;

    for x in 0..image.width() {
        for y in 0..image.height() {
            let (r, g, b, a) = image.get_pixel(x, y).channels4();
            if a > 10 {
                println!("PX {} {} {:02X}{:02X}{:02X}\n", x0+x, y0+y, r, g, b);
            }
        }
    }
}
