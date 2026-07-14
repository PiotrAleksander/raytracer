use std::rc::Rc;

use raytracing::{Camera, Color, HittableList, Lambertian, Point3, Sphere};

fn main() {
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));

    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));

    let camera = Camera::new(16.0 / 9.0, 400, 100, 10);
    camera.render(&world);
}
