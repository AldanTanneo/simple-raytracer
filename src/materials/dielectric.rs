use rand::Rng;

use super::{Material, ScatterResult};
use crate::hittable::HitRecord;
use crate::ray::{Ray, ScatteredRay};
use crate::vec3::color::Colour;
use crate::FastRng;

#[derive(Debug, Clone)]
pub struct Dielectric {
    pub attenuation: Colour,
    pub refraction_index: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, rng: &mut FastRng) -> ScatterResult {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = ray.direction.unit_vector();
        let cos_theta = unit_direction.dot(-hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let refracted_direction =
            if refraction_ratio * sin_theta > 1.0 || self.reflectance(cos_theta) > rng.gen() {
                unit_direction.reflect(hit_record.normal)
            } else {
                unit_direction.refract(hit_record.normal, refraction_ratio)
            };
        let new_ray = Ray::new(hit_record.point, refracted_direction);
        ScatterResult::Ray(ScatteredRay::new(new_ray, self.attenuation))
    }
}

impl Dielectric {
    pub fn new(attenuation: Colour, refraction_index: f64) -> Self {
        Self {
            attenuation,
            refraction_index,
        }
    }

    fn reflectance(&self, cos_theta: f64) -> f64 {
        let mut r0 = (1.0 - self.refraction_index) / (1.0 + self.refraction_index);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
    }
}
