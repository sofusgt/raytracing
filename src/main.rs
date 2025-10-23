use raytracing::Color;

fn main() {
    // Image
    let image_width: u32 = 256;
    let image_height: u32 = 256;

    // Initalize progress bar
    let progress_bar = indicatif::ProgressBar::new(image_width as u64 * image_height as u64);

    // Create image buffer and iterate pixels
    let mut image = image::ImageBuffer::new(image_width, image_height);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        // Generate color
        let color = Color::new(
            x as f32 / ((image_width - 1) as f32),
            y as f32 / ((image_height - 1) as f32),
            0.0,
        );

        // Assign pixel value
        let Color(color_data) = color;
        *pixel = image::Rgb(color_data);

        // Increment progress bar when we are done generating pixel
        progress_bar.inc(1);
    }
    progress_bar.finish();

    // Convert image from f32 to u8
    let image = image::DynamicImage::ImageRgb32F(image).into_rgb8();

    // Save image as png
    image.save("image.png").unwrap();
}
