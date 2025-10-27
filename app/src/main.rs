use std::{env, io};

mod bouncing_spheres;
mod checkered_spheres;
mod cornell_box;
mod cornell_smoke;
mod earth;
mod perlin_spheres;
mod quads;
mod simple_light;

fn main() -> Result<(), io::Error> {
    let arg = env::args().nth(1).unwrap_or_default();
    let idx = arg.parse::<i32>().unwrap_or(1);

    match idx {
        1 => bouncing_spheres::bouncing_spheres(),
        2 => checkered_spheres::checkered_spheres(),
        3 => earth::earth(),
        4 => perlin_spheres::perlin_spheres(),
        5 => quads::quads(),
        6 => simple_light::simple_light(),
        7 => cornell_box::cornell_box(),
        8 => cornell_smoke::cornell_smoke(),
        _ => Err(io::Error::other("oh no!")),
    }
}
