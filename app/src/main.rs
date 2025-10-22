use std::time::Instant;

fn main() {
    // Image
    let image_width: usize = 256;
    let image_height: usize = 256;

    // Start timer
    let now = Instant::now();

    // Render
    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.;

            let r = (255.999 * r) as u8;
            let g = (255.999 * g) as u8;
            let b = (255.999 * b) as u8;

            println!("{} {} {}\n", r, g, b);
        }
    }

    // End timer
    let elapsed = now.elapsed();
    eprintln!("Elapsed time: {}ms", elapsed.as_millis());
}
