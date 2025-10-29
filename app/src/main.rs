use std::io;

use ray_tracing_core::{Color, PixelProcessor, PnmImage, Rgb, color};

struct BaseProcessor {
    image_width: u32,
    image_height: u32,
}

impl BaseProcessor {
    fn new(image_width: u32, image_height: u32) -> Self {
        Self {
            image_width,
            image_height,
        }
    }
}

impl PixelProcessor for BaseProcessor {
    fn process_pixel(&self, i: u32, j: u32) -> Rgb {
        let pixel_color = Color::new(
            i as f64 / (self.image_width - 1) as f64,
            j as f64 / (self.image_height - 1) as f64,
            0.,
        );

        color::translate_color(pixel_color)
    }

    fn width(&self) -> u32 {
        self.image_width
    }

    fn height(&self) -> u32 {
        self.image_height
    }
}

fn main() -> Result<(), io::Error> {
    let image_width = 256_u32;
    let image_height = 256_u32;

    let processor = BaseProcessor::new(image_width, image_height);
    let mut ppm = PnmImage::new_ppm_ascii();

    ppm.generate(processor);
    // ppm.write_to_file("images/out.ppm")?;
    ppm.write_to_stdout()?;

    Ok(())
}
