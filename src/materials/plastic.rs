use rand::Rng;

use super::{Material, ScatterResult};
use crate::hittable::HitRecord;
use crate::ray::{Ray, ScatteredRay};
use crate::vec3::color::Colour;
use crate::vec3::Vec3;
use crate::FastRng;

#[derive(Clone, Debug)]
pub struct Plastic {
    pub albedo: Colour,
    pub roughness: f64,
}

impl Material for Plastic {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, rng: &mut FastRng) -> ScatterResult {
        let unit_direction = ray.direction.unit_vector();
        let cos_theta = unit_direction.dot(-hit_record.normal).min(1.0);

        let reflected = unit_direction.reflect(hit_record.normal);

        let new_direction = if self.reflectance(cos_theta) > rng.gen() {
            reflected + Vec3::random_in_unit_sphere(rng) * self.roughness
        } else {
            let mut scatter_direction = hit_record.normal + Vec3::random_in_unit_sphere(rng);
            if scatter_direction.near_zero() {
                scatter_direction = hit_record.normal;
            }
            scatter_direction
        };

        let new_ray = Ray::new(hit_record.point, new_direction);
        ScatterResult::Ray(ScatteredRay::new(new_ray, self.albedo))
    }
}

impl Plastic {
    pub fn new(albedo: Colour, roughness: f64) -> Self {
        Self { albedo, roughness }
    }

    /// Schlick's approximation with eta = 1.5, ie r0 = 0.04
    fn reflectance(&self, cos_theta: f64) -> f64 {
        0.04 + 0.96 * (1.0 - cos_theta).powi(5)
    }
}
