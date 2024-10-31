use clap::{command, Parser};

use crate::{Point3, Vec3};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long = "ratio-width", default_value_t = 16.)]
    pub ratio_width: f64,

    #[arg(long = "ratio-height", default_value_t = 9.)]
    pub ratio_height: f64,

    #[arg(long = "image-width", default_value_t = 400)]
    pub image_width: u32,

    #[arg(long = "samples-per-pixel", default_value_t = 10)]
    pub samples_per_pixel: u32,

    #[arg(long = "max-depth", default_value_t = 50)]
    pub max_depth: u32,

    #[arg(long = "field-of-view", default_value_t = 90)]
    pub vfov: u32,

    #[arg(long = "look-from", default_value = "-2,2,1")]
    pub lookfrom: Point3,

    #[arg(long = "look-at", default_value = "0,0,-1")]
    pub lookat: Point3,

    #[arg(long = "v-up", default_value = "0,1,0")]
    pub vup: Vec3,
}
