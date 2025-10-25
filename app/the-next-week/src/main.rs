use std::{env, io};

mod bouncing_spheres;
mod checkered_spheres;
mod earth;

fn main() -> Result<(), io::Error> {
    let arg = env::args().nth(1).unwrap_or_default();
    let idx = arg.parse::<i32>().unwrap_or(1);

    match idx {
        1 => bouncing_spheres::bouncing_spheres(),
        2 => checkered_spheres::checkered_spheres(),
        3 => earth::earth(),
        _ => Ok(()),
    }
}
