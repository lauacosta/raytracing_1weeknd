use crate::{dot, Hittable, Point3};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    #[must_use]
    pub fn new(center: &Point3, radius: f64) -> Self {
        Self {
            center: *center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        ray: &crate::Ray,
        ray_tmin: f64,
        ray_tmax: f64,
        record: &mut crate::HitRecord,
    ) -> bool {
        let oc = self.center - *ray.origin();
        let a = ray.direction().length_squared();
        let h = dot(*ray.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0. {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;

        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }

        record.t = root;
        record.point = ray.at(record.t);
        let outward_normal = (record.point - self.center) / self.radius;
        record.set_face_normal(ray, &outward_normal);

        true
    }
}