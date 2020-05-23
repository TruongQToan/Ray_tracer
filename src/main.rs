mod vector;
mod ray;
mod hittable;
mod utilities;
mod camera;
mod material;

use vector::{Color, unit_vector, Point3, Vec3, random, random_in_range};
use ray::Ray;
use camera::Camera;
use rand::prelude::*;

fn ray_color(r: Ray, world: Box<dyn hittable::Hittable>, depth: i16) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(ref rec) = world.hit(&r, 0.001, core::f64::MAX) {
        if let Some((attenuation, scattered)) = rec.mat.scatter(&r, rec) {
            return attenuation * ray_color(scattered, world, depth - 1);
        }

        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_dir = unit_vector(r.dir());
    let t = 0.5 * (unit_dir.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn random_scene() -> hittable::HittableList {
    let mut world = hittable::HittableList::new();
    let ground_material = material::Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Box::new(hittable::Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(ground_material))));
    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center: Point3 = Point3::new(a as f64 + 0.9 * rng.gen::<f64>(), 0.2, b as f64 + 0.9 * rng.gen::<f64>());
            if (center - Vec3::new(4.0, 0.2, 0.0)).l2() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = (random() * random()) as Color;
                    let sphere_material = material::Lambertian::new(albedo);
                    world.add(Box::new(hittable::Sphere::new(center, 0.2, Box::new(sphere_material))))
                } else if choose_mat < 0.95 {
                    let albedo = random_in_range(0.5, 1.0) as Color;
                    let fuzz = rng.gen_range(0.0, 0.5);
                    let sphere_material = material::Metal::new(albedo, fuzz);
                    world.add(Box::new(hittable::Sphere::new(center, 0.2, Box::new(sphere_material))))
                } else {
                    let sphere_material = material::Dielectric::new(1.5);
                    world.add(Box::new(hittable::Sphere::new(center, 0.2, Box::new(sphere_material))))
                }
            }
        }
    }
    
    let material1 = material::Dielectric::new(1.5);
    world.add(Box::new(hittable::Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, Box::new(material1))));

    let material2 = material::Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Box::new(hittable::Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, Box::new(material2))));

    let material3 = material::Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Box::new(hittable::Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, Box::new(material3))));
    
    return world;
}

fn main() {
    const IMAGE_WIDTH: u16 = 384;
    const MAX_DEPTH: i16 = 50;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    let image_height: u16 = (IMAGE_WIDTH as f64 / ASPECT_RATIO).floor() as u16;

    println!("P3\n{} {}\n255", IMAGE_WIDTH, image_height);

    let world = random_scene();
    
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.2;
    let cam = Camera::new(look_from, look_at, vup, 20.0, ASPECT_RATIO, aperture, dist_to_focus);
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
