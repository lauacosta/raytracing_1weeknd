use std::sync::Arc;

use clap::Parser;
use raytracing::{
    random_f64, random_f64_range, Args, Camera, Color, Dielectric, HittableList, Lambertian, Metal,
    Point3, Sphere, PI,
};

fn main() {
    let args = Args::parse();
    let aspect_ratio = args.ratio_width / args.ratio_height;
    let image_width = args.image_width;
    let samples_per_pixel = args.samples_per_pixel;
    let max_depth = args.max_depth;
    let vfov = args.vfov;
    let lookfrom = args.lookfrom;
    let lookat = args.lookat;
    let vup = args.vup;
    let defocus_angle = args.defocus_angle;
    let focus_dist = args.focus_dist;
    let mut cam = Camera::setup(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
        lookfrom,
        lookat,
        vup,
        focus_dist,
        defocus_angle,
    );

    let mut world = HittableList::default();
    let material_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));

    world.add(Sphere::new(
        &Point3::new(0., -1000., 0.),
        1000.,
        material_ground,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = random_f64();
            let center = Point3::new(
                f64::from(a) + 0.9 * random_f64(),
                0.2,
                f64::from(b) + 0.9 * random_f64(),
            );

            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                if choose_material < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Sphere::new(&center, 0.2, sphere_material));
                } else if choose_material < 0.95 {
                    let albedo = Color::random_with_range(0.5, 1.);
                    let fuzz = random_f64_range(0., 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Sphere::new(&center, 0.2, sphere_material));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Sphere::new(&center, 0.2, sphere_material));
                }
            }
        }
    }

    let material_1 = Arc::new(Dielectric::new(1.5));
    let material_2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let material_3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.));

    world.add(Sphere::new(&Point3::new(0., 1., 0.), 1.0, material_1));
    world.add(Sphere::new(&Point3::new(-4., 1., 0.), 1.0, material_2));
    world.add(Sphere::new(&Point3::new(4., 1., 0.), 1.0, material_3));

    cam.render(world);
}
