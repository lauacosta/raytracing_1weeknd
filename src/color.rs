use crate::{Interval, Vec3};

pub type Color = Vec3;

pub fn write_color(pixel_color: Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let r = linear_to_gamma(r);
    let g = linear_to_gamma(g);
    let b = linear_to_gamma(b);

    let intensity = Interval::new(0.000, 0.999);

    let rbyte = (256. * intensity.clamp(r)) as u64;
    let gbyte = (256. * intensity.clamp(g)) as u64;
    let bbyte = (256. * intensity.clamp(b)) as u64;

    println!("{rbyte}  {gbyte}  {bbyte}");
}

#[inline]
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0. {
        return linear_component.sqrt();
    }
    0.
}
