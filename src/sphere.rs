use std::sync::Arc;

use crate::{dot, Hittable, Interval, Material, Point3};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Material>,
}

impl Sphere {
    #[must_use]
    pub fn new(center: &Point3, radius: f64, mat: Arc<impl Material + 'static>) -> Self {
        Self {
            center: *center,
            radius: radius.max(0.0),
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::Ray, ray_t: Interval, record: &mut crate::HitRecord) -> bool {
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

        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        record.t = root;
        record.p = ray.at(record.t);
        let outward_normal = (record.p - self.center) / self.radius;
        record.set_face_normal(ray, &outward_normal);
        record.mat = self.mat.clone();

        true
    }
}
