use std::{io, sync::Arc};

use ray_tracing_core::prelude::*;

pub fn earth() -> Result<(), io::Error> {
    eprintln!("Running earth...");

    let earth_texture = Arc::new(ImageTexture::new("assets/earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::new(earth_texture));
    let globe = Arc::new(Sphere::new(Point3::zero(), 2., Some(earth_surface)));

    Camera::builder()
        .set_aspect_ratio(16. / 9.)
        .set_image_width(400)
        .set_samples_per_pixel(100)
        .set_max_depth(50)
        .set_vertical_view_angle(20.)
        .set_look_from(Point3::new(0., 0., 12.))
        .set_look_at(Point3::zero())
        .set_vup(Vec3::with_y(1.))
        .set_defocus_angle(0.)
        .set_focus_distance(10.)
        .build()
        .render(globe)?;

    Ok(())
}
