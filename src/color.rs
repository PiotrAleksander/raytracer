use crate::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel_color: Color) {
    let rbyte = (pixel_color.x * 255.999) as u8;
    let gbyte = (pixel_color.y * 255.999) as u8;
    let bbyte = (pixel_color.z * 255.999) as u8;

    println!("{} {} {}", rbyte, gbyte, bbyte);
}
