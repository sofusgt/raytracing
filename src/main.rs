fn main() {
    // Image
    let image_width = 256;
    let image_height = 256;

    print!("P3\n{image_width} {image_height}\n255\n");
    for j in 0..image_height {
        for i in 0..image_width {
            let r = i as f32 / (image_width as f32 - 1.0);
            let g = j as f32 / (image_height as f32 - 1.0);
            let b = 0.0;

            let r = (255.999 * r) as i32;
            let g = (255.999 * g) as i32;
            let b = (255.999 * b) as i32;

            print!("{r} {g} {b}\n");
        }
    }
}
