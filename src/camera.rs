use std::sync::atomic::{AtomicUsize, Ordering};

use rayon::prelude::*;

use crate::{
    degrees_to_radians, random_range,
    vec3::{random_in_unit_disk, unit_vector},
    write_color, Color, Hittable, HittableList, Interval, Point3, Ray, Vec3,
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
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

pub struct CameraConfig {
    pub aspect_ratio: f64,
    pub image_width: usize,
    pub samples_per_pixel: usize,
    pub max_depth: usize,
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
}

impl Camera {
    pub fn new(config: CameraConfig) -> Self {
        let image_height = ((config.image_width as f64 / config.aspect_ratio) as usize).max(1);

        let center = config.lookfrom;

        let theta = degrees_to_radians(config.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * config.focus_dist;
        let viewport_width = viewport_height * (config.image_width as f64 / image_height as f64);

        let w = unit_vector(config.lookfrom - config.lookat);
        let u = unit_vector(config.vup.cross(w));
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / config.image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;
        let viewport_upper_left =
            center - (config.focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius =
            config.focus_dist * degrees_to_radians(config.defocus_angle / 2.0).tan();

        Self {
            image_width: config.image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel: config.samples_per_pixel,
            max_depth: config.max_depth,
            pixel_samples_scale: 1.0 / config.samples_per_pixel as f64,
            defocus_angle: config.defocus_angle,
            defocus_disk_u: u * defocus_radius,
            defocus_disk_v: v * defocus_radius,
        }
    }

    pub fn render(&self, world: &HittableList) {
        println!("P3\n{} {} 255", self.image_width, self.image_height);

        let scanlines_done = AtomicUsize::new(0);
        let rows: Vec<Vec<Color>> = (0..self.image_height)
            .into_par_iter()
            .map(|j| {
                let row: Vec<Color> = (0..self.image_width)
                    .map(|i| {
                        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                        for _ in 0..self.samples_per_pixel {
                            let r = self.get_ray(i, j);
                            pixel_color += ray_color(&r, self.max_depth, world);
                        }
                        self.pixel_samples_scale * pixel_color
                    })
                    .collect();
                let done = scanlines_done.fetch_add(1, Ordering::Relaxed) + 1;
                eprint!("\rScanlines remaining: {}   ", self.image_height - done);
                row
            })
            .collect();

        for row in rows {
            for pixel in row {
                write_color(pixel);
            }
        }

        eprintln!("\nDone.");
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }
}

fn sample_square() -> Vec3 {
    Vec3::new(random_range(-0.5, 0.5), random_range(-0.5, 0.5), 0.0)
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
