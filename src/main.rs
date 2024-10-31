use clap::Parser;
use raytracing::{Args, Camera, HittableList, Point3, Sphere};

fn main() {
    let args = Args::parse();
    let aspect_ratio = args.ratio_width / args.ratio_height;
    let image_width = args.image_width;
    let samples_per_pixel = args.samples_per_pixel;
    let max_depth = args.max_depth;
    let mut cam = Camera::setup(aspect_ratio, image_width, samples_per_pixel, max_depth);

    let mut world = HittableList::default();
    world.add(Sphere::new(&Point3::new(0., 0., -1.), 0.5));
    world.add(Sphere::new(&Point3::new(0., -100.5, -1.), 100.0));

    cam.render(world);
}
