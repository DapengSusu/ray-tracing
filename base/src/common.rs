#[derive(Debug, Default)]
pub struct Degrees(f64);

#[derive(Debug, Default)]
pub struct Radians(f64);

impl From<Degrees> for Radians {
    fn from(degrees: Degrees) -> Self {
        Radians(degrees.0.to_radians())
    }
}

impl From<Radians> for Degrees {
    fn from(radians: Radians) -> Self {
        Degrees(radians.0.to_degrees())
    }
}
