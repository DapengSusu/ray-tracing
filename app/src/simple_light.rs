use std::{io, sync::Arc};

use ray_tracing_core::prelude::*;

pub fn simple_light() -> Result<(), io::Error> {
    eprintln!("Running Simple Light...");

    let pertext = TextureType::new_noise(4.);
    let difflight = Arc::new(MaterialType::new_diff_light_from_rgb(4., 4., 4.));

    let world = HittableObject::new_hittable_list(vec![
        HittableObject::new_sphere(
            Point3::with_y(-1000.),
            1000.,
            Arc::new(MaterialType::new_lamb_with_tex(pertext.clone())),
        ),
        HittableObject::new_sphere(
            Point3::with_y(2.),
            2.,
            Arc::new(MaterialType::new_lamb_with_tex(pertext)),
        ),
        HittableObject::new_quad(
            Point3::new(3., 1., -2.),
            Vec3::with_x(2.),
            Vec3::with_y(2.),
            difflight.clone(),
        ),
        HittableObject::new_sphere(Point3::new(0., 7., 0.), 2., difflight),
    ]);

    Camera::builder()
        .set_aspect_ratio(16. / 9.)
        .set_image_width(400)
        .set_samples_per_pixel(100)
        .set_max_depth(50)
        .set_background(Color::zero())
        .set_vertical_view_angle(20.)
        .set_look_from(Point3::new(26., 3., 6.))
        .set_look_at(Point3::with_y(2.))
        .set_vup(Vec3::with_y(1.))
        .set_defocus_angle(0.)
        .set_focus_distance(10.)
        .build()
        .render(Arc::new(world))?;

    Ok(())
}
