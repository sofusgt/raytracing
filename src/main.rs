fn main() {
    // Image
    let image_width = 256;
    let image_height = 256;

    let mut image = image::ImageBuffer::new(image_width, image_height);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let r = x as f32 / (image_width as f32 - 1.0);
        let g = y as f32 / (image_height as f32 - 1.0);
        let b = 0.0;

        let r = (255.999 * r) as u8;
        let g = (255.999 * g) as u8;
        let b = (255.999 * b) as u8;

        *pixel = image::Rgb([r, g, b]);
    }

    image.save("image.png").unwrap();
}
