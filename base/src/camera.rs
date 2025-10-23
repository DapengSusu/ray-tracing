use std::{
    io::{self, BufWriter, Write},
    sync::{
        Arc, LazyLock,
        atomic::{AtomicU32, Ordering},
    },
    time::Instant,
};

use rayon::prelude::*;

use crate::{common, prelude::*};

#[derive(Debug)]
pub struct Camera {
    /// Ratio of image width over height
    aspect_ratio: f64,
    /// Rendered image width in pixel count
    image_width: u32,
    /// Rendered image height
    image_height: u32,
    /// Count of random samples for each pixel
    samples_per_pixel: u32,
    /// Color scale factor for a sum of pixel samples
    pixel_samples_scale: f64,
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

fn sample_square() -> Vec3 {
    // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
    Vec3::with_xy(common::random() - 0.5, common::random() - 0.5)
}

static INTENSITY: LazyLock<Interval> = LazyLock::new(|| Interval::new(0., 0.999));

// Translate a color into a tuple of bytes
fn translate_color(pixel_color: Color) -> (u8, u8, u8) {
    let (r, g, b) = pixel_color.into();

    // translate the [0, 1] component values to the byte range [0, 255]
    (
        (255. * INTENSITY.clamp(r)) as u8,
        (255. * INTENSITY.clamp(g)) as u8,
        (255. * INTENSITY.clamp(b)) as u8,
    )
}

impl Camera {
    /// Generate a new camera builder to construct a camera.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ray_tracing_base::Camera;
    /// let camera = Camera::builder()
    ///     .set_aspect_ratio(1.)
    ///     .set_image_width(100)
    ///     .set_samples_per_pixel(10)
    ///     .build();
    /// ```
    pub fn builder() -> Self {
        Self {
            aspect_ratio: 1.,
            image_width: 100,
            image_height: 0,
            samples_per_pixel: 10,
            pixel_samples_scale: 0.,
            center: Point3::zero(),
            pixel00_loc: Point3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
        }
    }

    /// Set the aspect ratio of the camera.
    pub fn set_aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    /// Set the image width of the camera.
    pub fn set_image_width(mut self, image_width: u32) -> Self {
        self.image_width = image_width;
        self
    }

    /// Set the samples per pixel of the camera.
    pub fn set_samples_per_pixel(mut self, samples_per_pixel: u32) -> Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    /// Build the camera at last.
    ///
    /// * Initialize the camera.
    pub fn build(self) -> Self {
        self.initialize()
    }

    /// Render the scene with the given world.
    ///
    /// # Note
    ///
    /// You should call `build()` before calling this method.
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
                        let pixel_color: Color = (0..self.samples_per_pixel)
                            .map(|_| {
                                let ray = self.sample_ray(i, j);

                                ray_color(&ray, world.clone())
                            })
                            .sum();

                        translate_color(self.pixel_samples_scale * pixel_color)
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
        if (self.aspect_ratio - 0.).abs() < f64::EPSILON {
            panic!("Aspect ratio cannot be zero");
        }

        if self.image_width == 0 {
            panic!("Image width cannot be zero");
        }

        // Calculate the image height, and ensure that it's at least 1.
        self.image_height = ((self.image_width as f64 / self.aspect_ratio) as u32).max(1);

        self.pixel_samples_scale = 1. / self.samples_per_pixel as f64;

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

    // Construct a camera ray originating from the origin and directed
    // at randomly sampled point around the pixel location i, j.
    fn sample_ray(&self, i: u32, j: u32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + (i as f64 + offset.x) * self.pixel_delta_u
            + (j as f64 + offset.y) * self.pixel_delta_v;

        Ray::new(self.center, pixel_sample - self.center)
    }
}
