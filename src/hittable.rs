use crate::{dot, Interval, Point3, Ray, Vec3};

#[derive(Default, Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        let front_face = dot(*ray.direction(), *outward_normal) < 0.0;
        self.normal = if front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool;
}
