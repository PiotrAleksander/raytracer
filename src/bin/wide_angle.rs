use std::rc::Rc;

use raytracing::{Camera, Color, HittableList, Lambertian, Point3, Sphere};

fn main() {
    let mut world = HittableList::default();

    let r = (std::f64::consts::PI / 4.0).cos();

    let material_left = Rc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let material_right = Rc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    world.add(Box::new(Sphere::new(
        Point3::new(-r, 0.0, -1.0),
        r,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(r, 0.0, -1.0),
        r,
        material_right,
    )));

    let cam = Camera::new(16.0 / 9.0, 400, 100, 50, 90.0);
    cam.render(&world);
}
