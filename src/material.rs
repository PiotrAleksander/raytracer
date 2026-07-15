use crate::{
    vec3::{random_unit_vector, reflect, unit_vector},
    Color, HitRecord, Ray,
};

pub struct Scatter {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some(Scatter {
            attenuation: self.albedo,
            scattered: Ray::new(rec.p, scatter_direction),
        })
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let reflected = reflect(r_in.direction, rec.normal);
        let fuzzed_reflected = unit_vector(reflected) + (self.fuzz * random_unit_vector());
        Some(Scatter {
            attenuation: self.albedo,
            scattered: Ray::new(rec.p, fuzzed_reflected),
        })
    }
}
