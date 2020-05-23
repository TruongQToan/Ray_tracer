use crate::vector::{Vec3, Point3, unit_vector, cross, random_in_unit_disk};
use crate::ray::Ray;
use crate::utilities;

#[allow(dead_code)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    len_radius: f64,
}

impl Camera {
    pub fn new(lookfrom: Point3,
               lookat: Point3,
               vup: Vec3,
               vfov: f64,
               aspect_ratio: f64,
               aperture: f64,
               focus_dist: f64,
    ) -> Self {
        let theta: f64 = utilities::degrees_to_radians(vfov);
        let h: f64 = (theta / 2.0).tan();

        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = viewport_height * aspect_ratio;

        let w = unit_vector(lookfrom - lookat);
        let u = unit_vector(cross(vup, w));
        let v = cross(w, u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist*w;
        Self { origin: origin, 
            lower_left_corner: lower_left_corner, 
            horizontal: horizontal, 
            vertical: vertical,
            u: u,
            v: v,
            w: w,
            len_radius: aperture/2.0
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd: Point3 = self.len_radius * random_in_unit_disk();
        let offset: Vec3 = rd.x() * self.u + rd.y() * self.v;
        Ray::new(self.origin+offset, 
                 self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset
        )
    }
}