use std::{
    io::{self},
    sync::Arc,
};

use ray_tracing_base::{Camera, HittableList, Point3, Sphere};

fn main() -> Result<(), io::Error> {
    // World
    let world = Arc::new(HittableList::from_hittables(vec![
        Arc::new(Sphere::new(Point3::with_z(-1.), 0.5)),
        Arc::new(Sphere::new(Point3::with_yz(-100.5, -1.), 100.)),
    ]));

    // Camera render
    Camera::builder()
        .set_aspect_ratio(16. / 9.)
        .set_image_width(400)
        .set_samples_per_pixel(100)
        .build()
        .render(world)?;

    Ok(())
}
