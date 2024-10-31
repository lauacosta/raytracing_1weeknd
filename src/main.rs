use std::sync::Arc;

use clap::Parser;
use raytracing::{Args, Camera, Color, HittableList, Lambertian, Metal, Point3, Sphere};

fn main() {
    let args = Args::parse();
    let aspect_ratio = args.ratio_width / args.ratio_height;
    let image_width = args.image_width;
    let samples_per_pixel = args.samples_per_pixel;
    let max_depth = args.max_depth;
    let mut cam = Camera::setup(aspect_ratio, image_width, samples_per_pixel, max_depth);

    let mut world = HittableList::default();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.)));

    let material_right_small = Arc::new(Metal::new(Color::new(0.1, 0.2, 0.5)));

    let material_center_small = Arc::new(Lambertian::new(Color::new(2., 2.8, 2.8)));

    let material_left = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_center = Arc::new(Metal::new(Color::new(0.5, 0.5, 0.5)));
    let material_right = Arc::new(Lambertian::new(Color::new(1., 0.1, 0.1)));

    let material_above_left = Arc::new(Metal::new(Color::new(0.5, 0.5, 0.5)));
    let material_above_center = Arc::new(Lambertian::new(Color::new(0.1, 1., 0.2)));
    let material_above_right = Arc::new(Metal::new(Color::new(0.5, 0.5, 0.5)));

    world.add(Sphere::new(
        &Point3::new(0., -100.5, -1.),
        100.,
        material_ground,
    ));

    world.add(Sphere::new(
        &Point3::new(-1., -0.1, -1.),
        0.4,
        material_left,
    ));

    world.add(Sphere::new(
        &Point3::new(0., 0., -1.5),
        0.5,
        material_center,
    ));

    world.add(Sphere::new(
        &Point3::new(1., -0.1, -2.),
        0.4,
        material_right,
    ));

    world.add(Sphere::new(
        &Point3::new(-0.3, -0.3, -0.7),
        0.2,
        material_center_small,
    ));

    world.add(Sphere::new(
        &Point3::new(1., -0.2, -1.),
        0.3,
        material_right_small,
    ));

    world.add(Sphere::new(
        &Point3::new(-1., 0.9, -2.),
        0.4,
        material_above_left,
    ));

    world.add(Sphere::new(
        &Point3::new(0., 1., -1.5),
        0.4,
        material_above_center,
    ));

    world.add(Sphere::new(
        &Point3::new(1., 0.9, -1.),
        0.4,
        material_above_right,
    ));

    cam.render(world);
}
