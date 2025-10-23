use std::{
    f64::consts::PI,
    io::{self},
    sync::Arc,
};

use ray_tracing_base::prelude::*;

fn main() -> Result<(), io::Error> {
    let r = (PI / 4.).cos();

    let material_left = Arc::new(Lambertian::new(Color::with_z(1.)));
    let material_right = Arc::new(Lambertian::new(Color::with_x(1.)));

    // World
    let world = Arc::new(HittableList::from_hittables(vec![
        Arc::new(Sphere::new(
            Point3::with_xz(-r, -1.),
            r,
            Some(material_left),
        )),
        Arc::new(Sphere::new(
            Point3::with_xz(r, -1.),
            r,
            Some(material_right),
        )),
    ]));

    // Camera render
    Camera::builder()
        .set_aspect_ratio(16. / 9.)
        .set_image_width(400)
        .set_samples_per_pixel(100)
        .set_max_depth(50)
        .set_vertical_view_angle(90.)
        .build()
        .render(world)?;

    Ok(())
}
