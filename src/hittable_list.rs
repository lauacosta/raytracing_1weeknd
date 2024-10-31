use std::sync::Arc;

use crate::{HitRecord, Hittable, Interval};

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, hittable: impl Hittable + 'static) {
        self.objects.push(Arc::new(hittable));
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::Ray, ray_t: Interval, record: &mut crate::HitRecord) -> bool {
        let mut temp_record = HitRecord::default();
        let mut hit_anything = false;

        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(
                ray,
                Interval::new(ray_t.min, closest_so_far),
                &mut temp_record,
            ) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *record = temp_record;
            }
        }

        hit_anything
    }
}
