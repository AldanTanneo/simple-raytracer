#![allow(dead_code)]
pub mod dielectric;
pub mod emissive;
pub mod lambertian;
pub mod metal;

use core::fmt::Debug;

use crate::hittable::HitRecord;
use crate::ray::{Ray, ScatteredRay};
use crate::vec3::color::Colour;
use crate::FastRng;

#[derive(Debug, Clone)]
pub enum ScatterResult {
    Ray(ScatteredRay),
    Emissive(Colour),
    Absorbed,
}

pub trait Material: Debug + Send + Sync {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, rng: &mut FastRng) -> ScatterResult;
}
