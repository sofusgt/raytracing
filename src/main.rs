fn main() {
    // Image
    let image_width = 256;
    let image_height = 256;

    let mut image = image::ImageBuffer::new(image_width, image_height);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let r = x as f32 / (image_width as f32 - 1.0);
        let g = y as f32 / (image_height as f32 - 1.0);
        let b = 0.0;

        let ir = (255.0 * g) as u8;
        let ig = (255.0 * r) as u8;
        let ib = (255.0 * b) as u8;

        *pixel = image::Rgb([ir, ig, ib]);
    }

    image.save("image.png").unwrap();
}
