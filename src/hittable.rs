use std::rc::Rc;

use crate::{Interval, Material, Point3, Ray, Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        p: Point3,
        t: f64,
        r: &Ray,
        outward_normal: Vec3,
        material: Rc<dyn Material>,
    ) -> Self {
        let front_face = r.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            p,
            t,
            front_face,
            normal,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
