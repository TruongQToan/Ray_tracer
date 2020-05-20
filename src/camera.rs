use crate::vector::{Vec3, Point3};
use crate::ray::Ray;

#[allow(dead_code)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio: f64 = 16.0 / 9.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * aspect_ratio;
        let focal_length: f64 = 1.0;
        
        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
        Self { origin: origin, lower_left_corner: lower_left_corner, horizontal: horizontal, vertical: vertical}
    }
    
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner+u*self.horizontal+v*self.vertical-self.origin)
    }
}