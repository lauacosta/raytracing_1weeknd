use clap::Parser;
use raytracing::{dot, unit_vector, write_color, Args, Color, Point3, Ray, Vec3};

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = *center - *ray.origin();
    let a = ray.direction().length_squared();
    let h = dot(*ray.direction(), oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (h - discriminant.sqrt()) / a;
    }
}

fn ray_color(ray: Ray) -> Color {
    let t = hit_sphere(&Point3::new(0., 0., -1.), 0.5, &ray);

    if t > 0.0 {
        let n = unit_vector(ray.at(t) - Vec3::new(0., 0., -1.));
        let result = 0.5 * Color::new(n.x() + 1., n.y() + 1., n.z() + 1.);
        return result;
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
        let image_height = (image_width as f64 / aspect_ratio) as u64;
        if image_height < 1 {
            1
        } else {
            image_height
        }
    };

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

    print!("P3\n {}  {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {}  ", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixer_delta_u) + (j as f64 * pixer_delta_v);

            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(ray);
            write_color(pixel_color);
        }
    }
    eprintln!("Done");
}
