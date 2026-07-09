use crate::Interval;
use crate::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel_color: Color) {
    const INTENSITY: Interval = Interval::new(0.000, 0.999);
    let rbyte = (256.0 * INTENSITY.clamp(pixel_color.x)) as u8;
    let gbyte = (256.0 * INTENSITY.clamp(pixel_color.y)) as u8;
    let bbyte = (256.0 * INTENSITY.clamp(pixel_color.z)) as u8;

    println!("{} {} {}", rbyte, gbyte, bbyte);
}
