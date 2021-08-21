use super::{Material, ScatterResult};
use crate::hittable::HitRecord;
use crate::ray::{Ray, ScatteredRay};
use crate::vec3::color::Color;
use crate::vec3::Vec3;
use crate::FastRng;

#[derive(Clone, Debug)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord, rng: &mut FastRng) -> ScatterResult {
        let mut scatter_direction = hit_record.normal + Vec3::random_in_unit_sphere(rng);
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        let new_ray = Ray::new(hit_record.point, scatter_direction);
        ScatterResult::Ray(ScatteredRay::new(new_ray, self.albedo))
    }
}

impl Lambertian {
    pub const fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}
