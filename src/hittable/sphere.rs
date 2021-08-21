use crate::bounding_boxes::BoundingBox;
use crate::hittable::{Hit, HitRecord, Hittable};
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::Point3;
use crate::FastRng;

#[derive(Debug, Clone)]
pub struct Sphere<'a> {
    pub center: Point3,
    pub radius: f64,
    pub material: &'a Box<dyn Material + 'a>,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Point3, radius: f64, material: &'a Box<dyn Material + 'a>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, _: &mut FastRng) -> Hit {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let delta = half_b.powi(2) - a * c;
        if delta < 0.0 {
            return None;
        }

        let sqrtd = delta.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - self.center) / self.radius;
        Some(HitRecord::new(
            point,
            outward_normal,
            root,
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
