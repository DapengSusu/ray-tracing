use std::{
    io::{self},
    sync::Arc,
};

use the_next_week_core::prelude::*;

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
                    let material = Arc::new(Lambertian::from_color(albedo));
                    let center_end = center + Vec3::with_y(common::random_range(0., 0.5));
                    world.add(Arc::new(Sphere::new_moving(
                        center,
                        center_end,
                        0.2,
                        Some(material),
                    )));
                } else if material_random < 0.9 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.);
                    let fuzz = common::random_range(0., 0.5);
                    let material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, Some(material))));
                } else {
                    // glass
                    let material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, Some(material))));
                }
            }
        });
    });

    world
}

fn main() -> Result<(), io::Error> {
    // World
    let mut world = generate_sphere_random();

    let checker = Arc::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::with_isotropic(0.9),
    ));
    world.add(Arc::new(Sphere::new(
        Point3::with_y(-1000.),
        1000.,
        Some(Arc::new(Lambertian::new(checker))),
    )));

    let material_ground = Arc::new(Lambertian::from_color(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::with_y(-1000.),
        1000.,
        Some(material_ground),
    )));

    let material_major_1 = Arc::new(Dielectric::new(1.5));
    let material_major_2 = Arc::new(Lambertian::from_color(Color::new(0.4, 0.2, 0.1)));
    let material_major_3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.));

    world.add(Arc::new(Sphere::new(
        Point3::with_y(1.),
        1.,
        Some(material_major_1),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.,
        Some(material_major_2),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(4., 1., 0.),
        1.,
        Some(material_major_3),
    )));

    world = HittableList::from_hittable(Arc::new(BvhNode::from_hittable_list(world)));

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
