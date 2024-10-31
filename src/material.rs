use crate::{random_unit_vector, reflect, Color, HitRecord, Ray};

pub trait Material {
    fn scatter(
        &self,
        r_in: &mut Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        false
    }
}

#[derive(Default)]
pub struct Placeholder;

impl Material for Placeholder {}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &mut Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = record.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        *scattered = Ray::new(record.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}
impl Material for Metal {
    fn scatter(
        &self,
        r_in: &mut Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(*r_in.direction(), record.normal);
        *scattered = Ray::new(record.p, reflected);
        *attenuation = self.albedo;
        true
    }
}
