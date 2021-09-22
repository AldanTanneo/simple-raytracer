use super::{Material, ScatterResult};
use crate::hittable::HitRecord;
use crate::ray::{Ray, ScatteredRay};
use crate::vec3::color::Colour;
use crate::vec3::Vec3;
use crate::FastRng;

#[derive(Clone, Debug)]
pub struct Metal {
    pub albedo: Colour,
    pub fuzziness: f32,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, rng: &mut FastRng) -> ScatterResult {
        let reflected = ray.direction.reflect(hit_record.normal);
        let new_ray = Ray::new(
            hit_record.point,
            reflected + Vec3::random_in_unit_sphere(rng) * self.fuzziness,
        );
        if reflected.dot(hit_record.normal) > 0.0 && !new_ray.direction.near_zero() {
            ScatterResult::Ray(ScatteredRay::new(new_ray, self.albedo))
        } else {
            ScatterResult::Absorbed
        }
    }
}

impl Metal {
    pub fn new(albedo: Colour, fuzziness: f32) -> Self {
        Self { albedo, fuzziness }
    }
}
