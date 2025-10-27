use std::{io, sync::Arc};

use ray_tracing_core::prelude::*;

pub fn final_scene(
    image_width: u32,
    samples_per_pixel: u32,
    max_depth: u32,
) -> Result<(), io::Error> {
    eprintln!("Running Final Scene...");

    const BOXES_PER_SIDE: usize = 20;

    let mut boxes1 = HittableList::with_capacity(BOXES_PER_SIDE * BOXES_PER_SIDE);

    let ground = Arc::new(MaterialType::new_lamb_from_rgb(0.48, 0.83, 0.53));

    (0..BOXES_PER_SIDE).for_each(|i| {
        (0..BOXES_PER_SIDE).for_each(|j| {
            let w = 100.;
            let x0 = -1000. + i as f64 * w;
            let z0 = -1000. + j as f64 * w;
            let y0 = 0.;
            let x1 = x0 + w;
            let y1 = common::random_range(1., 101.);
            let z1 = z0 + w;

            boxes1.add(quad::new_box(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            ));
        });
    });

    let mut world = HittableList::from_hittable(HittableObject::new_bvh_node(boxes1));

    let difflight = Arc::new(MaterialType::new_diff_light_from_rgb(7., 7., 7.));
    world.add(HittableObject::new_quad(
        Point3::new(123., 554., 147.),
        Vec3::with_x(300.),
        Vec3::with_z(265.),
        difflight,
    ));

    let center1 = Point3::new(400., 400., 200.);
    let center2 = center1 + Vec3::with_x(30.);
    let sphere_material = Arc::new(MaterialType::new_diff_light_from_rgb(0.7, 0.3, 0.1));
    world.add(HittableObject::new_sphere_moving(
        center1,
        center2,
        50.,
        sphere_material,
    ));
    world.add(HittableObject::new_sphere(
        Point3::new(260., 150., 45.),
        50.,
        Arc::new(MaterialType::new_dielectric(1.5)),
    ));
    world.add(HittableObject::new_sphere(
        Point3::new(0., 150., 145.),
        50.,
        Arc::new(MaterialType::new_metal_from_rgb(0.8, 0.8, 0.9, 1.)),
    ));

    let boundary = HittableObject::new_sphere(
        Point3::new(360., 150., 145.),
        70.,
        Arc::new(MaterialType::new_dielectric(1.5)),
    );
    world.add(boundary.clone());
    world.add(HittableObject::new_cons_mid_with_color(
        Arc::new(boundary),
        0.2,
        Color::new(0.2, 0.4, 0.9),
    ));
    let boundary = HittableObject::new_sphere(
        Point3::zero(),
        5000.,
        Arc::new(MaterialType::new_dielectric(1.5)),
    );
    world.add(HittableObject::new_cons_mid_with_color(
        Arc::new(boundary),
        0.0001,
        Color::one(),
    ));

    let emat = Arc::new(MaterialType::new_lamb_with_tex(TextureType::new_image(
        "assets/earthmap.jpg",
    )));
    world.add(HittableObject::new_sphere(
        Point3::new(400., 200., 400.),
        100.,
        emat,
    ));

    let pertext = TextureType::new_noise(0.2);
    world.add(HittableObject::new_sphere(
        Point3::new(220., 280., 300.),
        80.,
        Arc::new(MaterialType::new_lamb_with_tex(pertext)),
    ));

    const NS: usize = 1000;

    let mut boxes2 = HittableList::with_capacity(NS);
    let white = Arc::new(MaterialType::new_lamb_from_rgb(0.73, 0.73, 0.73));

    (0..NS).for_each(|_| {
        boxes2.add(HittableObject::new_sphere(
            Point3::random_range(0., 165.),
            10.,
            white.clone(),
        ));
    });
    world.add(HittableObject::new_translate(
        Arc::new(HittableObject::new_rotate_y(
            Arc::new(HittableObject::new_bvh_node(boxes2)),
            15.,
        )),
        Vec3::new(-100., 270., 395.),
    ));

    Camera::builder()
        .set_aspect_ratio(1.)
        .set_image_width(image_width)
        .set_samples_per_pixel(samples_per_pixel)
        .set_max_depth(max_depth)
        .set_background(Color::zero())
        .set_vertical_view_angle(40.)
        .set_look_from(Point3::new(478., 278., -600.))
        .set_look_at(Point3::new(278., 278., 0.))
        .set_vup(Vec3::with_y(1.))
        .set_defocus_angle(0.)
        .set_focus_distance(10.)
        .build()
        .render(Arc::new(HittableObject::HittableList(world)))?;

    Ok(())
}
