use std::{
    io::{self, BufWriter, Write},
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
    time::Instant,
};

use ray_tracing_base::{Color, Hittable, HittableList, Point3, Ray, Sphere, Vec3};
use rayon::prelude::*;

/// aspect ratio, like 16:9, 4:3, etc.
const ASPECT_RATIO: f64 = 16. / 9.;

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = *center - ray.origin;
    let a = ray.direction.length_squared();
    let h = ray.direction.dot(&oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant >= 0. {
        (h - discriminant.sqrt()) / a
    } else {
        -1.
    }
}

/// Return the color for a given scene ray
fn ray_color<H: Hittable>(ray: &Ray, world: Arc<H>) -> Color {
    if let Some(hit) = world.hit(ray, 0., f64::INFINITY) {
        return 0.5 * (hit.normal + Color::one());
    }

    let t = hit_sphere(&Point3::from_z(-1.), 0.5, ray);
    if t > 0. {
        let normal = (ray.at(t) - Vec3::from_z(-1.)).to_unit();
        return 0.5 * Color::from_xyz(normal.x + 1., normal.y + 1., normal.z + 1.);
    }

    let direction = ray.direction.to_unit();
    let a = 0.5 * (direction.y + 1.);

    (1. - a) * Color::one() + a * Color::from_xyz(0.5, 0.7, 1.)
}

/// Translate a color into a tuple of bytes
fn translate_color(pixel_color: Color) -> (u8, u8, u8) {
    let (r, g, b) = pixel_color.into();

    // translate the [0, 1] component values to the byte range [0, 255]
    (
        (254.999 * r) as u8,
        (254.999 * g) as u8,
        (254.999 * b) as u8,
    )
}

fn main() -> Result<(), io::Error> {
    // Image
    let image_width: u32 = 400;
    // Calculate the image height, and ensure that it's at least 1.
    let image_height = (image_width as f64 / ASPECT_RATIO) as u32;
    let image_height = image_height.max(1);

    // World
    let world = Arc::new(HittableList::from_hittables(vec![
        Arc::new(Sphere::new(Point3::from_z(-1.), 0.5)),
        Arc::new(Sphere::new(Point3::from_yz(-100.5, -1.), 100.)),
    ]));

    // Camera
    let focal_length = 1.;
    let viewport_height = 2.;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::zero();

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::from_x(viewport_width);
    let viewport_v = Vec3::from_y(-viewport_height);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width;
    let pixel_delta_v = viewport_v / image_height;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::from_z(focal_length) - viewport_u / 2 - viewport_v / 2;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Writer
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    // Remaining lines
    let remaining_lines = AtomicU32::new(image_height);

    // Start timer
    let now = Instant::now();

    // Render
    writer.write_all(b"P3\n")?;
    writer.write_all(format!("{image_width} {image_height}\n").as_bytes())?;
    writer.write_all(b"255\n")?;

    let world = world.clone();
    let rows = (0..image_height)
        .into_par_iter() // rayon parallelize
        .map(|j| {
            let row = (0..image_width)
                .into_par_iter() // rayon parallelize
                .map(|i| {
                    let pixel_center = pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v);
                    let ray_direction = pixel_center - camera_center;
                    let ray = Ray::new(camera_center, ray_direction);

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
