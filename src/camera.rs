use rand::RngExt;

use crate::{
    degrees_to_radians, vec3::unit_vector, write_color, Color, Hittable, HittableList, Interval,
    Point3, Ray, Vec3,
};

pub struct Camera {
    image_width: usize,
    image_height: usize,
    samples_per_pixel: usize,
    max_depth: usize,

    pixel_samples_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: usize,
        samples_per_pixel: usize,
        max_depth: usize,
        vfov: f64,
    ) -> Self {
        let image_height = ((image_width as f64 / aspect_ratio) as usize).max(1);

        let center = Point3::new(0.0, 0.0, 0.0);

        let focal_length = 1.0;
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            max_depth,
            pixel_samples_scale: 1.0 / samples_per_pixel as f64,
        }
    }

    pub fn render(&self, world: &HittableList) {
        println!("P3\n{} {} 255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprintln!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += ray_color(&r, self.max_depth, world);
                }
                write_color(self.pixel_samples_scale as f64 * pixel_color);
            }
        }

        eprintln!("Done.");
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }
}

fn sample_square() -> Vec3 {
    Vec3::new(
        rand::rng().random_range(-0.5..0.5),
        rand::rng().random_range(-0.5..0.5),
        0.0,
    )
}

fn ray_color(r: &Ray, depth: usize, world: &HittableList) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
        if let Some(scatter) = rec.material.scatter(r, &rec) {
            return scatter.attenuation * ray_color(&scatter.scattered, depth - 1, world);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction = unit_vector(r.direction);
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}
