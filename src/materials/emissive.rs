use super::{Material, ScatterResult};
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::color::Colour;
use crate::FastRng;

#[derive(Debug, Clone)]
pub struct Emissive {
    color: Colour,
    intensity: f64,
}

impl Emissive {
    pub fn new(color: Colour, intensity: f64) -> Self {
        Self { color, intensity }
    }
}

impl Material for Emissive {
    fn scatter(&self, _: &Ray, _: &HitRecord, _: &mut FastRng) -> ScatterResult {
        ScatterResult::Emissive(self.color * self.intensity)
    }
}
