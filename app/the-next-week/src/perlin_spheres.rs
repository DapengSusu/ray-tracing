use std::{io, sync::Arc};

use the_next_week_core::prelude::*;

pub fn perlin_spheres() -> Result<(), io::Error> {
    eprintln!("Running Perlin Spheres...");

    let texture_perlin = Arc::new(NoiseTexture::new());

    let world = HittableList::from_hittables(vec![
        Arc::new(Sphere::new(
            Point3::with_y(-1000.),
            1000.,
            Some(Arc::new(Lambertian::new(texture_perlin.clone()))),
        )),
        Arc::new(Sphere::new(
            Point3::with_y(2.),
            2.,
            Some(Arc::new(Lambertian::new(texture_perlin))),
        )),
    ]);

    Camera::builder()
        .set_aspect_ratio(16. / 9.)
        .set_image_width(400)
        .set_samples_per_pixel(100)
        .set_max_depth(50)
        .set_vertical_view_angle(20.)
        .set_look_from(Point3::new(13., 2., 3.))
        .set_look_at(Point3::zero())
        .set_vup(Vec3::with_y(1.))
        .set_defocus_angle(0.)
        .set_focus_distance(10.)
        .build()
        .render(Arc::new(world))?;

    Ok(())
}
