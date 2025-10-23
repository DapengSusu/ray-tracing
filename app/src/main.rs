use std::{
    io::{self},
    sync::Arc,
};

use ray_tracing_base::prelude::*;

fn main() -> Result<(), io::Error> {
    let material_ground = Arc::new(Lambertian::new(Color::with_xyz(0.8, 0.8, 0.)));
    let material_center = Arc::new(Lambertian::new(Color::with_xyz(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Metal::new(Color::with_xyz(0.8, 0.8, 0.8), 0.3));
    let material_right = Arc::new(Metal::new(Color::with_xyz(0.8, 0.6, 0.2), 1.));

    // World
    let world = Arc::new(HittableList::from_hittables(vec![
        Arc::new(Sphere::new(
            Point3::with_yz(-100.5, -1.),
            100.,
            Some(material_ground),
        )),
        Arc::new(Sphere::new(
            Point3::with_z(-1.2),
            0.5,
            Some(material_center),
        )),
        Arc::new(Sphere::new(
            Point3::with_xz(-1., -1.),
            0.5,
            Some(material_left),
        )),
        Arc::new(Sphere::new(
            Point3::with_xz(1., -1.),
            0.5,
            Some(material_right),
        )),
    ]));

    // Camera render
    Camera::builder()
        .set_aspect_ratio(16. / 9.)
        .set_image_width(400)
        .set_samples_per_pixel(100)
        .set_max_depth(50)
        .build()
        .render(world)?;

    Ok(())
}
