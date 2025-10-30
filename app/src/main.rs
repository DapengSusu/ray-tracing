use std::{io, sync::Arc};

use ray_tracing_core::{
    Color, Hittable, HittableObject, Interval, PnmImage, Point3, Ray, Renderer, Rgb, Vec3,
};

struct Render<'a> {
    camera_center: &'a Vec3,
    pixel00_loc: &'a Vec3,
    pixel_delta_u: &'a Vec3,
    pixel_delta_v: &'a Vec3,
    world: Arc<HittableObject>,
}

impl<'a> Renderer for Render<'a> {
    fn render(&self, i: u32, j: u32) -> Rgb {
        let pixel_center =
            *self.pixel00_loc + (i as f64 * *self.pixel_delta_u) + (j as f64 * *self.pixel_delta_v);
        let ray_direction = pixel_center - *self.camera_center;
        let r = Ray::new(*self.camera_center, ray_direction);

        // 将 pixel color 转换为 Rgb
        ray_color(&r, self.world.clone()).into()
    }
}

const HIT_RAY_T: Interval = Interval {
    min: 0.001,
    max: f64::INFINITY,
};

/// Return the color for a given scene ray
fn ray_color<H: Hittable>(r: &Ray, world: Arc<H>) -> Color {
    if let Some(rec) = world.hit(r, &HIT_RAY_T) {
        return 0.5 * (rec.normal + Color::ONE);
    }

    let unit_direction = r.direction.to_unit();
    let a = 0.5 * (unit_direction.y + 1.);

    (1. - a) * Color::ONE + a * Color::new(0.5, 0.7, 1.)
}

fn main() -> Result<(), io::Error> {
    let aspect_ratio = 16. / 9.;
    let image_width = 400_u32;

    // Calculate the image height, and ensure that it's at least 1.
    let image_height = ((image_width as f64 / aspect_ratio) as u32).max(1);

    let world = Arc::new(HittableObject::new_list(vec![
        Arc::new(HittableObject::new_sphere(Point3::with_z(-1.), 0.5)),
        Arc::new(HittableObject::new_sphere(
            Point3::new(0., -100.5, -1.),
            100.,
        )),
    ]));

    let focal_length = 1.;
    let viewport_height = 2.;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Vec3::ZERO;

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0., 0.);
    let viewport_v = Vec3::new(0., -viewport_height, 0.);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::with_z(focal_length) - viewport_u / 2. - viewport_v / 2.;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let processor = Render {
        camera_center: &camera_center,
        pixel00_loc: &pixel00_loc,
        pixel_delta_u: &pixel_delta_u,
        pixel_delta_v: &pixel_delta_v,
        world,
    };
    let mut ppm = PnmImage::new_ppm_ascii(image_width, image_height);

    ppm.generate(processor);
    // ppm.write_to_file("images/out.ppm")?;
    ppm.write_to_stdout()?;

    Ok(())
}
