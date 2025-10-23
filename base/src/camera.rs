use std::{
    io::{self, BufWriter, Write},
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
    time::Instant,
};

use rayon::prelude::*;

use crate::{color, common, prelude::*};

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
    /// Maximum number of ray bounces into scene
    max_depth: u32,
    /// Vertical view angle (field of view)
    vfov: Degrees,
    /// Point camera is looking from
    look_from: Point3,
    /// Point camera is looking at
    look_at: Point3,
    /// Camera-relative "up" direction
    vup: Vec3,
    /// Variation angle of rays through each pixel
    defocus_angle: Degrees,
    /// Distance from camera lookfrom point to plane of perfect focus
    focus_dist: f64,
    /// Camera center
    center: Point3,
    /// Location of pixel 0, 0
    pixel00_loc: Point3,
    /// Offset to pixel to the right
    pixel_delta_u: Vec3,
    /// Offset to pixel below
    pixel_delta_v: Vec3,
    /// Defocus disk horizontal radius
    defocus_disk_u: Vec3,
    /// Defocus disk vertical radius
    defocus_disk_v: Vec3,
    /// Camera frame basis vectors
    basis: CameraBasis,
}

/// Camera frame basis vectors
#[derive(Debug, Default)]
struct CameraBasis {
    /// Camera-relative "right" direction
    u: Vec3,
    /// Camera-relative "up" direction
    v: Vec3,
    /// Camera-relative "forward" direction
    w: Vec3,
}

// Return the color for a given scene ray
fn ray_color<H: Hittable>(ray: Ray, depth: u32, world: Arc<H>) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Color::zero();
    }

    if let Some(hit) = world.hit(&ray, Interval::new(0.001, f64::INFINITY))
        && let Some(material) = &hit.material
    {
        if let Some((attenuation, scattered)) = material.scatter(&ray, &hit) {
            return attenuation * ray_color(scattered, depth - 1, world.clone());
        } else {
            return Color::zero();
        }
    }

    let direction = ray.direction.to_unit();
    let a = 0.5 * (direction.y + 1.);

    (1. - a) * Color::one() + a * Color::with_xyz(0.5, 0.7, 1.)
}

fn sample_square() -> Vec3 {
    // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
    Vec3::with_xy(common::random() - 0.5, common::random() - 0.5)
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
    ///     .set_max_depth(10)
    ///     .set_vertical_view_angle(90.)
    ///     .build();
    /// ```
    pub fn builder() -> Self {
        Self {
            aspect_ratio: 1.,
            image_width: 100,
            image_height: 0,
            samples_per_pixel: 10,
            pixel_samples_scale: 0.,
            max_depth: 10,
            vfov: Degrees(90.),
            look_from: Point3::zero(),
            look_at: Point3::with_z(-1.),
            vup: Vec3::with_y(1.),
            defocus_angle: Degrees(0.),
            focus_dist: 10.,
            center: Point3::zero(),
            pixel00_loc: Point3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
            defocus_disk_u: Vec3::zero(),
            defocus_disk_v: Vec3::zero(),
            basis: CameraBasis::default(),
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

    /// Set the maximum depth of the camera.
    pub fn set_max_depth(mut self, max_depth: u32) -> Self {
        self.max_depth = max_depth;
        self
    }

    /// Set the vertical view angle of the camera.
    pub fn set_vertical_view_angle(mut self, vfov: f64) -> Self {
        self.vfov = Degrees(vfov);
        self
    }

    /// Set the look from point of the camera.
    pub fn set_look_from(mut self, look_from: Point3) -> Self {
        self.look_from = look_from;
        self
    }

    /// Set the look at point of the camera.
    pub fn set_look_at(mut self, look_at: Point3) -> Self {
        self.look_at = look_at;
        self
    }

    /// Set the up direction of the camera.
    pub fn set_vup(mut self, vup: Vec3) -> Self {
        self.vup = vup;
        self
    }

    /// Set the defocus angle of the camera.
    pub fn set_defocus_angle(mut self, defocus_angle: f64) -> Self {
        self.defocus_angle = Degrees(defocus_angle);
        self
    }

    /// Set the focus distance of the camera.
    pub fn set_focus_distance(mut self, focus_distance: f64) -> Self {
        self.focus_dist = focus_distance;
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

                                ray_color(ray, self.max_depth, world.clone())
                            })
                            .sum();

                        color::translate_color(self.pixel_samples_scale * pixel_color)
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
        if self.aspect_ratio.abs() < f64::EPSILON {
            panic!("Aspect ratio cannot be zero");
        }

        if self.image_width == 0 {
            panic!("Image width cannot be zero");
        }

        // Calculate the image height, and ensure that it's at least 1.
        self.image_height = ((self.image_width as f64 / self.aspect_ratio) as u32).max(1);

        self.pixel_samples_scale = 1. / self.samples_per_pixel as f64;

        // Camera center
        self.center = self.look_from;

        // Determine viewport dimensions.
        let theta = self.vfov.to_radians();
        let h = (*theta / 2.).tan();
        let viewport_height = 2. * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        self.basis.w = (self.look_from - self.look_at).to_unit();
        self.basis.u = vec3::cross(&self.vup, &self.basis.w).to_unit();
        self.basis.v = vec3::cross(&self.basis.w, &self.basis.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * self.basis.u; // Vector across viewport horizontal edge
        let viewport_v = viewport_height * (-self.basis.v); // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - self.focus_dist * self.basis.w - viewport_u / 2 - viewport_v / 2;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_dist * (*self.defocus_angle / 2.).to_radians().tan();
        self.defocus_disk_u = defocus_radius * self.basis.u;
        self.defocus_disk_v = defocus_radius * self.basis.v;

        self
    }

    // Construct a camera ray originating from the origin and directed
    // at randomly sampled point around the pixel location i, j.
    fn sample_ray(&self, i: u32, j: u32) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.

        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + (i as f64 + offset.x) * self.pixel_delta_u
            + (j as f64 + offset.y) * self.pixel_delta_v;

        let ray_origin = if *self.defocus_angle <= 0. {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        // Returns a random point in the camera defocus disk.
        let p = Vec3::random_in_unit_disk();

        self.center + p.x * self.defocus_disk_u + p.y * self.defocus_disk_v
    }
}
