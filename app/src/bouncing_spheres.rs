use std::{
    io::{self},
    sync::Arc,
};

use ray_tracing_core::prelude::*;

fn generate_sphere_random() -> HittableList {
    let mut world = HittableList::new();

    (-10..11).for_each(|i| {
        (-10..11).for_each(|j| {
            let material_random = common::random();
            let center = Point3::new(
                j as f64 + 0.9 * common::random(),
                0.2,
                i as f64 + 0.9 * common::random(),
            );

            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                if material_random < 0.7 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let material = MaterialType::new_lamb_from_color(albedo);
                    let center_end = center + Vec3::with_y(common::random_range(0., 0.5));
                    world.add(HittableObject::new_sphere_moving(
                        center, center_end, 0.2, material,
                    ));
                } else if material_random < 0.9 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.);
                    let fuzz = common::random_range(0., 0.5);
                    let material = MaterialType::new_metal(albedo, fuzz);
                    world.add(HittableObject::new_sphere(center, 0.2, material));
                } else {
                    // glass
                    let material = MaterialType::new_dielectric(1.5);
                    world.add(HittableObject::new_sphere(center, 0.2, material));
                }
            }
        });
    });

    world
}

pub fn bouncing_spheres() -> Result<(), io::Error> {
    eprintln!("Running bouncing spheres...");

    // World
    let mut world = generate_sphere_random();

    let checker = TextureType::new_checker_from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::with_isotropic(0.9),
    );
    world.add(HittableObject::new_sphere(
        Point3::with_y(-1000.),
        1000.,
        MaterialType::new_lamb(checker),
    ));

    let material_ground = MaterialType::new_lamb_from_color(Color::new(0.5, 0.5, 0.5));
    world.add(HittableObject::new_sphere(
        Point3::with_y(-1000.),
        1000.,
        material_ground,
    ));

    let material_major_1 = MaterialType::new_dielectric(1.5);
    let material_major_2 = MaterialType::new_lamb_from_color(Color::new(0.4, 0.2, 0.1));
    let material_major_3 = MaterialType::new_metal(Color::new(0.7, 0.6, 0.5), 0.);

    world.add(HittableObject::new_sphere(
        Point3::with_y(1.),
        1.,
        material_major_1,
    ));
    world.add(HittableObject::new_sphere(
        Point3::new(-4., 1., 0.),
        1.,
        material_major_2,
    ));
    world.add(HittableObject::new_sphere(
        Point3::new(4., 1., 0.),
        1.,
        material_major_3,
    ));

    let world = HittableObject::new_bvh_node(world);

    // Camera render
    Camera::builder()
        .set_aspect_ratio(16. / 9.)
        .set_image_width(400)
        .set_samples_per_pixel(100)
        .set_max_depth(50)
        .set_vertical_view_angle(20.)
        .set_look_from(Point3::new(13., 2., 3.))
        .set_look_at(Point3::zero())
        .set_vup(Vec3::with_y(1.))
        .set_defocus_angle(0.6)
        .set_focus_distance(10.)
        .build()
        .render(Arc::new(world))?;

    Ok(())
}
