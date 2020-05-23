use crate::ray;
use crate::hittable;
use crate::vector;
use crate::utilities;

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
        let scatter_direction = rec.normal + vector::random_unit_vector();
        return Some((self.albedo, ray::Ray::new(rec.p, scatter_direction)));
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
        Self { albedo: vector::Color::default(), fuzz: 0.0 }
    }
}

impl Material for Metal {
    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(Metal::new(self.albedo, self.fuzz))
    }

    fn scatter(&self, r_in: &ray::Ray, rec: &hittable::HitRecord) -> Option<(vector::Color, ray::Ray)> {
        let reflected: vector::Vec3 = vector::reflect(&vector::unit_vector(r_in.dir()), &rec.normal);
        let scattered = ray::Ray::new(rec.p, reflected + self.fuzz * vector::random_in_unit_sphere());
        if vector::dot(&reflected, &rec.normal) > 0.0 {
            return Some((self.albedo, scattered));
        }

        return None;
    }
}

pub struct Dielectric {
    pub ref_idx: f64,
}

impl Dielectric {
    pub fn new(ri: f64) -> Self { Self { ref_idx: ri } }
}

impl Material for Dielectric {
    fn box_clone(&self) -> Box<dyn Material> { Box::new(Dielectric::new(self.ref_idx)) }

    fn scatter(&self, r_in: &ray::Ray, rec: &hittable::HitRecord) -> Option<(vector::Color, ray::Ray)> {
        let attenuation = vector::Color::new(1.0, 1.0, 1.0);
        let mut etai_over_etat = self.ref_idx;
        if rec.front_face {
            etai_over_etat = 1.0 / self.ref_idx;
        }

        let unit_direction = vector::unit_vector(r_in.dir());

        let cos_theta = vector::dot(&((-1.0) * unit_direction), &rec.normal).min(1.0);
        let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = vector::reflect(&unit_direction, &rec.normal);
            let scattered = ray::Ray::new(rec.p, reflected);
            return Some((attenuation, scattered));
        }

        let reflect_prob: f64 = schlick(cos_theta, self.ref_idx);
        if (utilities::random_double()) < reflect_prob {
            let reflected = vector::reflect(&unit_direction, &rec.normal);
            let scattered = ray::Ray::new(rec.p, reflected);
            return Some((attenuation, scattered));
        }

        let refracted = vector::refract(&unit_direction, &rec.normal, etai_over_etat);
        let scattered = ray::Ray::new(rec.p, refracted);
        return Some((attenuation, scattered));
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0: f64 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}
