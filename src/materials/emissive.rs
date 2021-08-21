use super::{Material, ScatterResult};
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::color::Color;
use crate::FastRng;

#[derive(Debug, Clone)]
pub struct Emissive {
    color: Color,
    intensity: f32,
}

impl Emissive {
    pub fn new(color: Color, intensity: f32) -> Self {
        Self { color, intensity }
    }
}

impl Material for Emissive {
    fn scatter(&self, _: &Ray, _: &HitRecord, _: &mut FastRng) -> ScatterResult {
        ScatterResult::Emissive(self.color * self.intensity)
    }
}
