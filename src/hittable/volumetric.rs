use rand::Rng;
use rand_distr::Normal;

use crate::bounding_boxes::BoundingBox;
use crate::hittable::{Hit, HitRecord, Hittable};
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::FastRng;

#[derive(Debug, Clone)]
pub struct Volume<'a> {
    pub center: Point3,
    pub radius: f64,
    pub density: f64,
    pub material: &'a (dyn Material + 'a),
    pub distribution: Normal<f64>,
}

impl<'a> Volume<'a> {
    pub fn new(
        center: Point3,
        radius: f64,
        density: f64,
        material: &'a (dyn Material + 'a),
    ) -> Self {
        Self {
            center,
            radius,
            density,
            material,
            distribution: Normal::new(0.5, 0.16).unwrap(),
        }
    }
}

impl<'a> Hittable for Volume<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rng: &mut FastRng) -> Hit {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let delta = half_b.powi(2) - a * c;
        if delta < 1e-8_f64 {
            return None;
        }

        let sqrtd = delta.sqrt();
        let mut root1 = (-half_b - sqrtd) / a;
        if root1 > t_max {
            return None;
        } else if root1 < t_min {
            root1 = t_min;
        }
        let root2 = (-half_b + sqrtd) / a;
        if root2 < t_min {
            return None;
        }
        let point1 = ray.at(root1);
        let point2 = ray.at(root2);

        let hit_chance = 0.25 * (point1 - point2).length_squared();

        if rng.gen::<f64>() * self.radius * self.radius > self.density * self.density * hit_chance {
            return None;
        }

        let random_pos: f64 = rng.gen();
        let time = (1.0 - random_pos) * root1 + random_pos * root2;
        let outward_normal = Vec3::random_in_unit_sphere(rng);
        Some(HitRecord::new(
            ray.at(time),
            outward_normal,
            time,
            ray,
            self.material,
        ))
    }

    fn bounding_box(&self) -> BoundingBox {
        BoundingBox {
            minimum: self.center - self.radius,
            maximum: self.center + self.radius,
        }
    }
}
