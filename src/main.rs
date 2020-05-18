mod vector;
mod ray;

use vector::{Point3, Vec3};
use ray::Ray;

fn main() {
    // const IMAGE_WIDTH: u16 = 256;
    // const IMAGE_HEIGHT: u16 = 256;
    // println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    // for j in (0..IMAGE_HEIGHT).rev() {
    //     for i in 0..IMAGE_WIDTH {
    //         let r: f64 = i as f64 / (IMAGE_WIDTH-1) as f64;
    //         let g: f64 = j as f64 / (IMAGE_HEIGHT-1) as f64;
    //         let b: f64 = 0.25;
    //
    //         let c = vector::vector::Color::new(r, g, b);
    //         print!("{}", c);
    //     }
    // }
    
    let origin = Point3::new(1.0,1.0,1.0);
    let dir = Vec3::new(2.0,2.0,2.0);
    let ray = Ray::new(origin, dir);
    println!("vector = {:?}", ray);
}
