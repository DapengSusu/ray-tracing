use std::{
    io::{self, BufWriter, Write},
    sync::atomic::{AtomicU32, Ordering},
    time::Instant,
};

// use ray_tracing_core::prelude::*;

use rayon::prelude::*;

fn main() -> Result<(), io::Error> {
    let image_width = 256_u32;
    let image_height = 256_u32;

    // Writer
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    // Remaining lines
    let remaining_lines = AtomicU32::new(image_height);

    // Start timer
    let now = Instant::now();

    // Render
    writer.write_all(b"P3\n")?;
    writer.write_all(format!("{} {}\n", image_width, image_height).as_bytes())?;
    writer.write_all(b"255\n")?;

    let rows = (0..image_height)
        .into_par_iter() // rayon parallelize
        .map(|j| {
            let row = (0..image_width)
                .into_par_iter() // rayon parallelize
                .map(|i| {
                    let r = i as f64 / (image_width - 1) as f64;
                    let g = j as f64 / (image_height - 1) as f64;
                    let b = 0.;

                    (
                        (255.999 * r) as u16,
                        (255.999 * g) as u16,
                        (255.999 * b) as u16,
                    )
                })
                .collect::<Vec<_>>();

            let remaining = remaining_lines.fetch_sub(1, Ordering::Relaxed);
            eprint!("\r\x1B[KScanlines remaining: {}", remaining - 1);

            let mut row_bytes = Vec::with_capacity(row.len() * 9);
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
