use crate::{Point3, Vec3, texture::HitRecord};

/// 二维图形，包括方形，三角形，圆
pub trait PlaneFigure: Send + Sync {
    fn update_bounding_box(self, q: &Point3, u: &Vec3, v: &Vec3) -> Self;

    fn is_interior(hit: HitRecord, a: f64, b: f64) -> Option<HitRecord>;
}
