use crate::bounding_boxes::BoundingBox;
use crate::hittable::{Hit, HitRecord, Hittable};
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::FastRng;

#[derive(Debug, Clone)]
pub struct Quad<'a> {
    pub vertex: Point3,
    pub edges: (Vec3, Vec3),
    pub material: &'a Box<dyn Material + 'a>,
}

impl<'a> Quad<'a> {
    pub fn new(vertex: Point3, edges: (Vec3, Vec3), material: &'a Box<dyn Material + 'a>) -> Self {
        Self {
            vertex,
            edges,
            material,
        }
    }

    pub fn vertices(&self) -> (Point3, Point3, Point3, Point3) {
        (
            self.vertex,
            self.vertex + self.edges.0,
            self.vertex + self.edges.1,
            self.vertex + self.edges.0 + self.edges.1,
        )
    }
}

impl<'a> Hittable for Quad<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, _: &mut FastRng) -> Hit {
        let u = ray.direction;
        let (v, w) = self.edges;
        let normal = v.cross(w);
        let determinant = normal.dot(u);
        if determinant.abs() < 1e-8_f64 {
            None
        } else {
            let a_minus_origin = self.vertex - ray.origin;
            let time = normal.dot(a_minus_origin) / determinant;
            let lambda = u.cross(w).dot(a_minus_origin) / determinant;
            let mu = u.cross(v).dot(-a_minus_origin) / determinant;
            if time < t_min
                || time > t_max
                || lambda <= 0.0
                || mu <= 0.0
                || lambda >= 1.0
                || mu >= 1.0
            {
                None
            } else {
                let point = ray.at(time);
                Some(HitRecord::new(
                    point,
                    normal.unit_vector(),
                    time,
                    ray,
                    self.material,
                ))
            }
        }
    }

    fn bounding_box(&self) -> BoundingBox {
        let vertices = self.vertices();
        BoundingBox {
            minimum: vertices.0.min(vertices.1).min(vertices.2).min(vertices.3) - 1e-7f64,
            maximum: vertices.0.max(vertices.1).max(vertices.2).max(vertices.3) + 1e-7f64,
        }
    }
}
