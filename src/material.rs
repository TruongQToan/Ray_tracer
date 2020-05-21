use crate::ray;
use crate::hittable;
use crate::vector;

pub trait Material {
    fn box_clone(&self) -> Box<dyn Material>;
    fn scatter(&self, r_in: &ray::Ray, rec: &hittable::HitRecord) -> Option<(vector::Color, ray::Ray)>;
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.box_clone()
    }    
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Lambertian {
    pub albedo: vector::Color,
}

impl Lambertian {
    pub fn new(albedo: vector::Color) -> Self {
        Self { albedo: albedo }
    }
    
    #[allow(dead_code)]
    pub fn default() -> Self {
        Self { albedo: vector::Color::default() }
    }

}

impl Material for Lambertian {
    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(Lambertian::new(self.albedo))     
    }
    
    fn scatter(&self, _: &ray::Ray, rec: &hittable::HitRecord) -> Option<(vector::Color, ray::Ray)> {
        let scatter_direction = rec.normal+vector::random_unit_vector();
        return Some((self.albedo, ray::Ray::new(rec.p, scatter_direction)))
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Metal {
    pub fuzz: f64,
    pub albedo: vector::Color,
}

impl Metal {
    pub fn new(albedo: vector::Color, f: f64) -> Self { 
        let fuzz: f64 = if f < 1.0 {
            f
        } else {
            1.0
        };
        
        Self { albedo: albedo, fuzz: fuzz }
    }
    
    #[allow(dead_code)]
    pub fn default() -> Self {
        Self { albedo: vector::Color::default(), fuzz: 0.0, }
    }
}

impl Material for Metal {
    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(Metal::new(self.albedo, self.fuzz))
    }
    
    fn scatter(&self, r_in: &ray::Ray, rec: &hittable::HitRecord) -> Option<(vector::Color, ray::Ray)> {
        let reflected: vector::Vec3 = vector::reflect(&vector::unit_vector(r_in.dir()), &rec.normal);
        let scattered = ray::Ray::new(rec.p, reflected+self.fuzz*vector::random_in_unit_sphere());
        if vector::dot(&reflected, &rec.normal) > 0.0 {
            return Some((self.albedo, scattered));
        }
        
        return None
    }
}