mod vector;
mod ray;

use vector::{Point3, Vec3, Color, unit_vector, dot};
use ray::Ray;

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin() - *center;
    let r_dir = &r.dir();
    let a = dot(r_dir, r_dir);
    let b = 2.0 * dot(&oc, r_dir);
    let c = dot(&oc, &oc) - radius*radius;
    let discriminant: f64 = b*b-4.0*a*c;
    return discriminant > 0.0;
}

fn ray_color(r: Ray) -> Color {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, &r) {
        return Color::new(1.0, 0.0, 0.0);
    }
    
    let unit_direction: Vec3 = unit_vector(r.dir());
    let t: f64 = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    const IMAGE_WIDTH: u16 = 384;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    let image_height: u16 = (IMAGE_WIDTH as f64 / ASPECT_RATIO).floor() as u16;

    println!("P3\n{} {}\n255", IMAGE_WIDTH, image_height);

    let viewport_height: f64 = 4.0;
    let viewport_width: f64 = ASPECT_RATIO * viewport_height;
    let focal_length: f64 = 1.0;

    let origin: Vec3 = Point3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner: Vec3 = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    for j in (0..image_height).rev() {
        for i in 0..IMAGE_WIDTH {
            let u: f64 = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v: f64 = j as f64 / (image_height - 1) as f64;
            let dir: Vec3 = lower_left_corner + u * horizontal + v * vertical - origin;
            let r: Ray = Ray::new(origin, dir);
            let c: Color = ray_color(r);
            print!("{}", c)
        }
    }

    // let origin = Point3::new(1.0, 1.0, 1.0);
    // let dir = Vec3::new(2.0, 2.0, 2.0);
    // let a_ray = Ray::new(origin, dir);
    // println!("vector = {:?}", origin);
    // println!("vector = {:?}", a_ray);
    // println!("vector = {:?}", &a_ray.at(16.0));
}
