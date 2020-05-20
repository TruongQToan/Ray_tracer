mod vector;
mod ray;
mod hittable;
mod utilities;
mod camera;

use vector::{Color, unit_vector};
use ray::Ray;
use camera::Camera;
use rand::prelude::*;

fn ray_color(r: Ray, world: &hittable::Hittable) -> Color {
    if let Some(ref x) = world.hit(&r, 0.0, core::f64::MAX) {
        return 0.5*(x.normal + Color::new(1.0, 1.0, 1.0));
    }
    
    let unit_dir = unit_vector(r.dir());
    let t = 0.5*(unit_dir.y()+1.0);
    return (1.0-t)*Color::new(1.0, 1.0, 1.0)+t*Color::new(0.5, 0.7, 1.0);
}

fn main() {
    const IMAGE_WIDTH: u16 = 384;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    let image_height: u16 = (IMAGE_WIDTH as f64 / ASPECT_RATIO).floor() as u16;

    println!("P3\n{} {}\n255", IMAGE_WIDTH, image_height);

    let mut world = hittable::HittableList::new();
    world.add(hittable::Sphere::new(vector::Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(hittable::Sphere::new(vector::Point3::new(0.0, -100.5, -1.0), 100.0));
    
    let cam = Camera::new();
    let mut rng = rand::thread_rng();
    
    for j in (0..image_height).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut color: vector::Color = vector::Color::new(0.0, 0.0, 0.0);
            for _s in 0..100 {
                let u: f64 = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v: f64 = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
                let ray = cam.get_ray(u, v);
                let c: Color = ray_color(ray, &world);
                color = color + c;
            }
            print!("{}", color)
        }
    }
}
