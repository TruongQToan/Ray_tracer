use crate::vector;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: vector::Point3, 
    dir: vector::Vec3,
}

impl Ray {
    #[allow(dead_code)]
    pub fn new(origin: vector::Point3, dir: vector::Vec3) -> Self {
        Self { origin, dir }
    }

    #[allow(dead_code)]
    pub fn origin(&self) -> vector::Point3 {
        self.origin
    }

    #[allow(dead_code)]
    pub fn dir(&self) -> vector::Vec3 {
        self.dir
    }

    #[allow(dead_code)]
    pub fn at(&self, t: f64) -> vector::Point3 {
        self.origin+t*self.dir
    }
}
