use std::{io, sync::Arc};

use ray_tracing_core::prelude::*;

pub fn perlin_spheres() -> Result<(), io::Error> {
    eprintln!("Running Perlin Spheres...");

    let texture_perlin = TextureType::new_noise(4.);

    let world = HittableObject::new_list(vec![
        HittableObject::new_sphere(
            Point3::with_y(-1000.),
            1000.,
            Arc::new(MaterialType::new_lamb(texture_perlin.clone())),
        ),
        HittableObject::new_sphere(
            Point3::with_y(2.),
            2.,
            Arc::new(MaterialType::new_lamb(texture_perlin)),
        ),
    ]);

    Camera::builder()
        .set_aspect_ratio(16. / 9.)
        .set_image_width(400)
        .set_samples_per_pixel(100)
        .set_max_depth(50)
        .set_background(Color::new(0.7, 0.8, 1.))
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
