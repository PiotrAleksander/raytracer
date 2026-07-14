use crate::{vec3::random_unit_vector, Color, HitRecord, Ray};

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
