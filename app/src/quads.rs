use std::{io, sync::Arc};

use ray_tracing_core::prelude::*;

pub fn quads() -> Result<(), io::Error> {
    eprintln!("Running Quads...");

    // Materials
    let left_red = MaterialType::new_lamb_from_color(Color::new(1., 0.2, 0.2));
    let back_green = MaterialType::new_lamb_from_color(Color::new(0.2, 1., 0.2));
    let right_blue = MaterialType::new_lamb_from_color(Color::new(0.2, 0.2, 1.));
    let upper_orange = MaterialType::new_lamb_from_color(Color::new(1., 0.5, 0.));
    let lower_teal = MaterialType::new_lamb_from_color(Color::new(0.2, 0.8, 0.8));

    let world = HittableObject::new_list(vec![
        HittableObject::new_quad(
            Point3::new(-3., -2., 5.),
            Vec3::new(0., 0., -4.),
            Vec3::new(0., 4., 0.),
            left_red,
        ),
        HittableObject::new_quad(
            Point3::new(-2., -2., 0.),
            Vec3::new(4., 0., 0.),
            Vec3::new(0., 4., 0.),
            back_green,
        ),
        HittableObject::new_quad(
            Point3::new(3., -2., 1.),
            Vec3::new(0., 0., 4.),
            Vec3::new(0., 4., 0.),
            right_blue,
        ),
        HittableObject::new_quad(
            Point3::new(-2., 3., 1.),
            Vec3::new(4., 0., 0.),
            Vec3::new(0., 0., 4.),
            upper_orange,
        ),
        HittableObject::new_quad(
            Point3::new(-2., -3., 5.),
            Vec3::new(4., 0., 0.),
            Vec3::new(0., 0., -4.),
            lower_teal,
        ),
    ]);

    Camera::builder()
        .set_aspect_ratio(1.)
        .set_image_width(400)
        .set_samples_per_pixel(100)
        .set_max_depth(50)
        .set_vertical_view_angle(80.)
        .set_look_from(Point3::new(0., 0., 9.))
        .set_look_at(Point3::zero())
        .set_vup(Vec3::with_y(1.))
        .set_defocus_angle(0.)
        .set_focus_distance(10.)
        .build()
        .render(Arc::new(world))?;

    Ok(())
}
