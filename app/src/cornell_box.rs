use std::{io, sync::Arc};

use ray_tracing_core::prelude::*;

pub fn cornell_box() -> Result<(), io::Error> {
    eprintln!("Running Cornell Box...");

    let mat_red = MaterialType::new_lamb_from_color(Color::new(0.65, 0.05, 0.05));
    let mat_white = MaterialType::new_lamb_from_color(Color::new(0.73, 0.73, 0.73));
    let mat_green = MaterialType::new_lamb_from_color(Color::new(0.12, 0.45, 0.15));
    let difflight = MaterialType::new_diff_light_from_color(Color::new(15., 15., 15.));

    let world = HittableObject::new_list(vec![
        HittableObject::new_quad(
            Point3::with_x(555.),
            Vec3::with_y(555.),
            Vec3::with_z(555.),
            mat_green,
        ),
        HittableObject::new_quad(
            Point3::zero(),
            Vec3::with_y(555.),
            Vec3::with_z(555.),
            mat_red,
        ),
        HittableObject::new_quad(
            Point3::new(343., 554., 332.),
            Vec3::with_x(-130.),
            Vec3::with_z(-105.),
            difflight,
        ),
        HittableObject::new_quad(
            Point3::zero(),
            Vec3::with_x(555.),
            Vec3::with_z(555.),
            mat_white.clone(),
        ),
        HittableObject::new_quad(
            Point3::new(555., 555., 555.),
            Vec3::with_x(-555.),
            Vec3::with_z(-555.),
            mat_white.clone(),
        ),
        HittableObject::new_quad(
            Point3::with_z(555.),
            Vec3::with_x(555.),
            Vec3::with_y(555.),
            mat_white,
        ),
    ]);

    Camera::builder()
        .set_aspect_ratio(1.)
        .set_image_width(600)
        .set_samples_per_pixel(200)
        .set_max_depth(50)
        .set_background(Color::zero())
        .set_vertical_view_angle(40.)
        .set_look_from(Point3::new(278., 278., -800.))
        .set_look_at(Point3::new(278., 278., 0.))
        .set_vup(Vec3::with_y(1.))
        .set_defocus_angle(0.)
        .set_focus_distance(10.)
        .build()
        .render(Arc::new(world))?;

    Ok(())
}
