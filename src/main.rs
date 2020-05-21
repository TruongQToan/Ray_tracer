mod vector;
mod ray;
mod hittable;
mod utilities;
mod camera;
mod material;

use vector::{Color, unit_vector};
use ray::Ray;
use camera::Camera;
use rand::prelude::*;

fn ray_color(r: Ray, world: Box<dyn hittable::Hittable>, depth: i16) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0)
    }
    
    if let Some(ref rec) = world.hit(&r, 0.001, core::f64::MAX) {
        if let Some((attenuation, scattered)) = rec.mat.scatter(&r, rec) {
            return attenuation*ray_color(scattered, world, depth-1)
        }
        
        return Color::new(0.0, 0.0, 0.0)
    }
    
    let unit_dir = unit_vector(r.dir());
    let t = 0.5*(unit_dir.y()+1.0);
    return (1.0-t)*Color::new(1.0, 1.0, 1.0)+t*Color::new(0.5, 0.7, 1.0);
}

fn main() {
    const IMAGE_WIDTH: u16 = 384;
    const MAX_DEPTH: i16 = 50;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    let image_height: u16 = (IMAGE_WIDTH as f64 / ASPECT_RATIO).floor() as u16;

    println!("P3\n{} {}\n255", IMAGE_WIDTH, image_height);

    let mut world = hittable::HittableList::new();
    world.add(Box::new(hittable::Sphere::new(
        vector::Point3::new(0.0, 0.0, -1.0), 0.5, 
        Box::new(material::Lambertian::new(vector::Color::new(0.7, 0.3, 0.3))))));
    world.add(Box::new(hittable::Sphere::new(
        vector::Point3::new(0.0, -100.5, -1.0), 100.0, 
        Box::new(material::Lambertian::new(vector::Color::new(0.8, 0.8, 0.0))))));
    world.add(Box::new(hittable::Sphere::new(
        vector::Point3::new(1.0, 0.0, -1.0), 0.5, 
        Box::new(material::Metal::new(vector::Color::new(0.8, 0.6, 0.2), 0.6)))));
    world.add(Box::new(hittable::Sphere::new(
        vector::Point3::new(-1.0, 0.0, -1.0), 0.5, 
        Box::new(material::Metal::new(vector::Color::new(0.8, 0.8, 0.8), 0.5)))));
    
    let cam = Camera::new();
    let mut rng = rand::thread_rng();
    
    for j in (0..image_height).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut color: vector::Color = vector::Color::new(0.0, 0.0, 0.0);
            for _s in 0..100 {
                let u: f64 = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v: f64 = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
                let ray = cam.get_ray(u, v);
                let c: Color = ray_color(ray, Box::new(world.clone()), MAX_DEPTH);
                color = color + c;
            }
            print!("{}", color)
        }
    }
}
