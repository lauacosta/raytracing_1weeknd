use crate::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel_color: Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let rbyte = (255.999 * r) as u64;
    let gbyte = (255.999 * g) as u64;
    let bbyte = (255.999 * b) as u64;

    println!("{rbyte}  {gbyte}  {bbyte}");
}
