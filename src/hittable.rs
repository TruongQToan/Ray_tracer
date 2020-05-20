use crate::{vector, ray};

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Default)]
pub struct HitRecord {
    pub p: vector::Vec3,
    pub normal: vector::Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        return Default :: default();
    }
    
    pub fn set_face_normal(&mut self, r: &ray::Ray, outward_normal: vector::Vec3) {
        self.front_face = vector::dot(&r.dir(), &outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal
        } else {
            self.normal = (-1.0)*outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: vector::Point3,    
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: vector::Point3, radius: f64) -> Self { Self { center, radius }}    
}

impl Hittable for Sphere {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: vector::Vec3 = r.origin() - self.center;
        let a: f64 = r.dir().l2_squared();
        let half_b: f64 = vector::dot(&oc, &r.dir());
        let c: f64 = oc.l2_squared() - self.radius*self.radius;
        let discriminant: f64 = half_b*half_b-a*c;
        
        if discriminant > 0.0 {
            let root: f64 = discriminant.sqrt();
            let temp: f64 = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let mut rec = HitRecord::new();
                rec.t = temp;
                rec.p = r.at(temp);
                let outward_normal: vector::Vec3 = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                return Some(rec)
            }

            let temp: f64 = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let mut rec = HitRecord::new();
                rec.t = temp;
                rec.p = r.at(temp);
                let outward_normal: vector::Vec3 = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                return Some(rec)
            }
        }
        
        return None;
    }
}

pub struct HittableList<T: Hittable> {
    pub objects: std::vec::Vec<T>,
}

impl<T: Hittable> HittableList<T> {
    pub fn new() -> Self {
        Self { objects: std::vec::Vec::new() }
    }
    
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    
    pub fn add(&mut self, object: T) {
        self.objects.push(object)
    }
    
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far: f64 = t_max;
        let mut temp_rec: Option<HitRecord> = None;

        for object in &self.objects {
            let rec = object.hit(r, t_min, closest_so_far);
            if let Some(x) = rec {
                closest_so_far = x.t;
                temp_rec = Some(x);
            }
        }

        return temp_rec;
    }
}
