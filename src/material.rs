use crate::{
    dot, random_f64, random_unit_vector, reflect, refract, unit_vector, Color, HitRecord, Ray, Vec3,
};

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
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
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
        let mut reflected = reflect(*r_in.direction(), record.normal);
        reflected = unit_vector(reflected) + (self.fuzz * random_unit_vector());
        *scattered = Ray::new(record.p, reflected);
        *attenuation = self.albedo;

        dot(*scattered.direction(), record.normal) > 0.
    }
}

pub struct Dielectric {
    pub refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }

    pub fn reflectance(&self, cosine: f64, refractive_index: f64) -> f64 {
        let mut r0 = (1. - refractive_index) / (1. + refractive_index);
        r0 = r0 * r0;
        r0 + (1. - r0) * f64::powi(1. - cosine, 5)
    }
}
impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &mut Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1., 1., 1.);
        let ri = if record.front_face {
            1. / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = unit_vector(*r_in.direction());

        let cos_theta = f64::min(dot(-unit_direction, record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let direction = if cannot_refract || self.reflectance(cos_theta, ri) > random_f64() {
            reflect(unit_direction, record.normal)
        } else {
            refract(unit_direction, record.normal, ri)
        };

        *scattered = Ray::new(record.p, direction);
        true
    }
}
