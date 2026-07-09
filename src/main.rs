use raytracing::{Camera, HittableList, Point3, Sphere};

fn main() {
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new(16.0 / 9.0, 400);
    camera.render(&world);
}
