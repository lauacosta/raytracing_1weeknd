use std::sync::Arc;

use clap::Parser;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use raytracing::{
    unit_vector, write_color, Args, Color, HitRecord, Hittable, HittableList, Point3, Ray, Sphere,
    Vec3, INFINITY,
};

fn ray_color(ray: &Ray, world: &impl Hittable) -> Color {
    let mut record = HitRecord::default();

    if world.hit(&ray, 0.0, INFINITY, &mut record) {
        return 0.5 * (record.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = unit_vector(*ray.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let args = Args::parse();

    let aspect_ratio = args.ratio_width / args.ratio_height;
    let image_width = args.image_width;

    let image_height = {
        let image_height = (image_width as f64 / aspect_ratio) as u32;
        if image_height < 1 {
            1
        } else {
            image_height
        }
    };

    let mut world = HittableList::default();

    world.add(Sphere::new(&Point3::new(0., 0., -1.), 0.5));
    world.add(Sphere::new(&Point3::new(0., -100.5, -1.), 100.0));

    dbg!(
        "aspect ratio {} - image width {} - image height {}",
        aspect_ratio,
        image_width,
        image_height
    );

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0., 0., 0.);

    let viewport_u = Vec3::new(viewport_width, 0., 0.);
    let viewport_v = Vec3::new(0., -viewport_height, 0.);

    let pixer_delta_u = viewport_u / image_width as f64;
    let pixer_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camera_center - Vec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixer_delta_u + pixer_delta_v);

    print!("P3\n {image_width}  {image_height}\n255\n");

    let style = ProgressStyle::default_bar();
    for j in (0..image_height).progress_with_style(style) {
        // bar.inc(1);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixer_delta_u) + (j as f64 * pixer_delta_v);

            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&ray, &world);
            write_color(pixel_color);
        }
    }
    // bar.finish();
}
