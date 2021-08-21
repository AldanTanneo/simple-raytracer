#![allow(dead_code)]

use crate::vec3::color::Color;
use crate::vec3::{Point3, Vec3};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub const fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}

#[derive(Clone, Debug)]
pub struct ScatteredRay {
    pub ray: Ray,
    pub attenuation: Color,
}

impl ScatteredRay {
    pub fn new(ray: Ray, attenuation: Color) -> Self {
        Self { ray, attenuation }
    }
}
