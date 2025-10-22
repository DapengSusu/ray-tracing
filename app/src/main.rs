use std::{
    io::{self, BufWriter, Write},
    sync::atomic::{AtomicUsize, Ordering},
    time::Instant,
};

use rayon::prelude::*;

fn main() -> Result<(), io::Error> {
    // Image
    let image_width: usize = 256;
    let image_height: usize = 256;

    // Start timer
    let now = Instant::now();

    // Writer
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    // Remaining lines
    let remaining_lines = AtomicUsize::new(image_height);

    // Render
    writer.write_all(b"P3\n")?;
    writer.write_all(format!("{image_width} {image_height}\n").as_bytes())?;
    writer.write_all(b"255\n")?;

    let rows = (0..image_height)
        .into_par_iter()
        .map(|j| {
            let row = (0..image_width)
                .map(|i| {
                    let r = i as f64 / (image_width - 1) as f64;
                    let g = j as f64 / (image_height - 1) as f64;
                    let b = 0.;

                    (
                        (255.999 * r) as u8,
                        (255.999 * g) as u8,
                        (255.999 * b) as u8,
                    )
                })
                .collect::<Vec<_>>();

            let remaining = remaining_lines.fetch_sub(1, Ordering::Relaxed);
            eprint!("\r\x1B[KScanlines remaining: {}", remaining - 1);

            let mut row_bytes = Vec::with_capacity(row.len() * 10);
            for (r, g, b) in &row {
                row_bytes.extend_from_slice(format!("{r} {g} {b}\n").as_bytes());
            }

            row_bytes
        })
        .collect::<Vec<_>>();

    for row in &rows {
        writer.write_all(row)?;
    }

    // End timer
    eprint!("\r\x1B[K");
    let elapsed = now.elapsed();
    eprintln!("\nDone. Elapsed time: {}ms", elapsed.as_millis());

    Ok(())
}
