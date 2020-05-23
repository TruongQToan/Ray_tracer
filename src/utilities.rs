use rand::prelude::*;

#[allow(dead_code)]
pub fn random_double() -> f64 {
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    return rng.gen::<f64>()    
}

#[allow(dead_code)]
pub fn random_in_range(min: f64, max: f64) -> f64 {
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    return rng.gen_range(min, max)
}

#[allow(dead_code)]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * std::f64::consts::PI / 180.0;
}

#[allow(dead_code)]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min
    }
    
    if x > max {
        return max
    }
    
    return x
}