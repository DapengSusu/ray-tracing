use std::{io, sync::Arc};

use ray_tracing_core::prelude::*;

pub fn cornell_smoke() -> Result<(), io::Error> {
    eprintln!("Running Cornell Smoke...");

    let mat_red = Arc::new(MaterialType::new_lamb_from_rgb(0.65, 0.05, 0.05));
    let mat_white = Arc::new(MaterialType::new_lamb_from_rgb(0.73, 0.73, 0.73));
    let mat_green = Arc::new(MaterialType::new_lamb_from_rgb(0.12, 0.45, 0.15));
    let difflight = Arc::new(MaterialType::new_diff_light_from_rgb(7., 7., 7.));

    let box1 = quad::new_box(
        Point3::zero(),
        Point3::new(165., 330., 165.),
        mat_white.clone(),
    );
    let box1 = Arc::new(HittableObject::new_rotate_y(Arc::new(box1), 15.));
    let box1 = Arc::new(HittableObject::new_translate(
        box1,
        Vec3::new(265., 0., 295.),
    ));

    let box2 = quad::new_box(
        Point3::zero(),
        Point3::new(165., 165., 165.),
        mat_white.clone(),
    );
    let box2 = Arc::new(HittableObject::new_rotate_y(Arc::new(box2), -18.));
    let box2 = Arc::new(HittableObject::new_translate(
        box2,
        Vec3::new(130., 0., 65.),
    ));

    let world = HittableObject::new_hittable_list(vec![
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
            Point3::new(113., 554., 127.),
            Vec3::with_x(330.),
            Vec3::with_z(305.),
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
            mat_white.clone(),
        ),
        HittableObject::new_cons_mid_with_color(box1, 0.01, Color::zero()),
        HittableObject::new_cons_mid_with_color(box2, 0.01, Color::one()),
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
