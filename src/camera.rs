use indicatif::{ProgressIterator, ProgressStyle};

use crate::{
    random_f64, random_unit_vector, unit_vector, write_color, Color, HitRecord, Hittable, Interval,
    Point3, Ray, Vec3, INFINITY,
};

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pixel_samples_scale: f64,
    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn setup(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> Self {
        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            ..Default::default()
        }
    }
    pub fn render(&mut self, world: impl Hittable) {
        self.initialize();
        print!("P3\n {}  {}\n255\n", self.image_width, self.image_height);

        let style = ProgressStyle::default_bar();
        for j in (0..self.image_height).progress_with_style(style) {
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0., 0., 0.);

                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += self.ray_color(&ray, self.max_depth, &world);
                }

                write_color(pixel_color * self.pixel_samples_scale);
            }
        }
    }

    fn initialize(&mut self) {
        self.image_height = {
            let image_height = (f64::from(self.image_width) / self.aspect_ratio) as u32;
            if image_height < 1 {
                1
            } else {
                image_height
            }
        };

        self.center = Point3::new(0., 0., 0.);

        self.pixel_samples_scale = 1.0 / f64::from(self.samples_per_pixel);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width =
            viewport_height * (f64::from(self.image_width) / f64::from(self.image_height));

        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        self.pixel_delta_u = viewport_u / f64::from(self.image_width);
        self.pixel_delta_v = viewport_v / f64::from(self.image_height);

        let viewport_upper_left =
            self.center - Vec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;

        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        eprintln!(
            "- aspect ratio {}\n- image width {}\n- image height {}",
            self.aspect_ratio, self.image_width, self.image_height
        );
    }

    fn ray_color(&self, ray: &Ray, max_depth: u32, world: &impl Hittable) -> Color {
        if max_depth == 0 {
            return Color::new(0., 0., 0.);
        }

        let mut record = HitRecord::default();

        if world.hit(ray, Interval::new(0.001, INFINITY), &mut record) {
            // let direction = random_on_hemisphere(&record.normal);
            let direction = record.normal + random_unit_vector();
            return 0.5 * self.ray_color(&Ray::new(record.p, direction), max_depth - 1, world);
        }

        let unit_direction = unit_vector(*ray.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((f64::from(i) + offset.x()) * self.pixel_delta_u)
            + ((f64::from(j) + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.)
    }
}
