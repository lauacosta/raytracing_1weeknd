use clap::{command, Parser};

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
}
