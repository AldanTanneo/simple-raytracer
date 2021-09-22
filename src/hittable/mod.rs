pub mod hittable_list;
pub mod quad;
pub mod sphere;
pub mod triangle;
pub mod volumetric;

use std::fmt::Debug;

use crate::bounding_boxes::BoundingBox;
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::FastRng;

#[derive(Clone)]
pub struct HitRecord<'a> {
    pub point: Point3,
    pub normal: Vec3,
    pub time: f64,
    pub front_face: bool,
    pub material: &'a (dyn Material + 'a),
}

pub type Hit<'a> = Option<HitRecord<'a>>;

impl<'a> HitRecord<'a> {
    pub fn new(
        point: Point3,
        outward_normal: Vec3,
        time: f64,
        ray: &Ray,
        material: &'a (dyn Material + 'a),
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            point,
            normal,
            time,
            front_face,
            material,
        }
    }
}

pub trait Hittable: Debug + Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rng: &mut FastRng) -> Hit;

    fn bounding_box(&self) -> BoundingBox;
}
