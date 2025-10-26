use std::{io, sync::Arc};

use ray_tracing_core::prelude::*;

pub fn checkered_spheres() -> Result<(), io::Error> {
    eprintln!("Running checkered spheres...");

    let checker = TextureType::new_checker_from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::with_isotropic(0.9),
    );

    let world = HittableObject::new_list(vec![
        HittableObject::new_sphere(
            Point3::with_y(-10.),
            10.,
            MaterialType::new_lamb(checker.clone()),
        ),
        HittableObject::new_sphere(Point3::with_y(10.), 10., MaterialType::new_lamb(checker)),
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
