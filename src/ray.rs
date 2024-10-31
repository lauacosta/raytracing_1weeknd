use crate::{Point3, Vec3};

#[derive(Default)]
pub struct Ray {
    origin: Point3,
    dir: Vec3,
}

impl Ray {
    #[must_use]
    pub fn new(origin: Point3, dir: Vec3) -> Self {
        Self { origin, dir }
    }

    #[must_use]
    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    #[must_use]
    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    #[must_use]
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.dir
    }
}
