use std::{
    io::{self, BufWriter, Write},
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
    time::Instant,
};

use rayon::prelude::*;

use crate::prelude::*;

#[derive(Debug)]
pub struct Camera {
    /// Ratio of image width over height
    aspect_ratio: f64,
    /// Rendered image width in pixel count
    image_width: u32,
    /// Rendered image height
    image_height: u32,
    /// Camera center
    center: Point3,
    /// Location of pixel 0, 0
    pixel00_loc: Point3,
    /// Offset to pixel to the right
    pixel_delta_u: Vec3,
    /// Offset to pixel below
    pixel_delta_v: Vec3,
}

// Return the color for a given scene ray
fn ray_color<H: Hittable>(ray: &Ray, world: Arc<H>) -> Color {
    if let Some(hit) = world.hit(ray, Interval::new(0., f64::INFINITY)) {
        return 0.5 * (hit.normal + Color::one());
    }

    let direction = ray.direction.to_unit();
    let a = 0.5 * (direction.y + 1.);

    (1. - a) * Color::one() + a * Color::with_xyz(0.5, 0.7, 1.)
}

// Translate a color into a tuple of bytes
fn translate_color(pixel_color: Color) -> (u8, u8, u8) {
    let (r, g, b) = pixel_color.into();

    // translate the [0, 1] component values to the byte range [0, 255]
    (
        (254.999 * r) as u8,
        (254.999 * g) as u8,
        (254.999 * b) as u8,
    )
}

impl Camera {
    /// Create a new camera with the given aspect ratio and image width.
    pub fn new(aspect_ratio: f64, image_width: u32) -> Self {
        Self {
            aspect_ratio,
            image_width,
            image_height: 1,
            center: Point3::zero(),
            pixel00_loc: Point3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
        }
        .initialize()
    }

    /// Render the scene with the given world.
    pub fn render<H: Hittable>(&mut self, world: Arc<H>) -> Result<(), io::Error> {
        // Writer
        let stdout = io::stdout();
        let mut writer = BufWriter::new(stdout.lock());

        // Remaining lines
        let remaining_lines = AtomicU32::new(self.image_height);

        // Start timer
        let now = Instant::now();

        // Render
        writer.write_all(b"P3\n")?;
        writer.write_all(format!("{} {}\n", self.image_width, self.image_height).as_bytes())?;
        writer.write_all(b"255\n")?;

        let world = world.clone();
        let rows = (0..self.image_height)
            .into_par_iter() // rayon parallelize
            .map(|j| {
                let row = (0..self.image_width)
                    .into_par_iter() // rayon parallelize
                    .map(|i| {
                        let pixel_center =
                            self.pixel00_loc + (i * self.pixel_delta_u) + (j * self.pixel_delta_v);
                        let ray_direction = pixel_center - self.center;
                        let ray = Ray::new(self.center, ray_direction);

                        let pixel_color = ray_color(&ray, world.clone());

                        translate_color(pixel_color)
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

    fn initialize(mut self) -> Self {
        // Calculate the image height, and ensure that it's at least 1.
        self.image_height = ((self.image_width as f64 / self.aspect_ratio) as u32).max(1);

        // Camera center
        self.center = Point3::zero();

        // Determine viewport dimensions.
        let focal_length = 1.;
        let viewport_height = 2.;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::with_x(viewport_width);
        let viewport_v = Vec3::with_y(-viewport_height);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - Vec3::with_z(focal_length) - viewport_u / 2 - viewport_v / 2;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        self
    }
}
