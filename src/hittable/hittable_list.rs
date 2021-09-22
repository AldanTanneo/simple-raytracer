use super::{Hit, Hittable};
use crate::bounding_boxes::BoundingBox;
use crate::ray::Ray;
use crate::FastRng;

#[derive(Debug, Default)]
pub struct HittableList<'a> {
    pub objects: Vec<Box<dyn Hittable + 'a>>,
}

impl<'a> Hittable for HittableList<'a> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rng: &mut FastRng) -> Hit {
        let mut record = None;
        let mut closest = t_max;

        for object in self.iter() {
            let tmp = object.hit(ray, t_min, closest, rng);

            if let Some(r) = tmp {
                closest = r.time;
                record = Some(r);
            }
        }

        record
    }

    fn bounding_box(&self) -> BoundingBox {
        self.iter()
            .skip(1)
            .fold(self[0].bounding_box(), |a, b| a.join(&b.bounding_box()))
    }
}

impl<'a> std::ops::Deref for HittableList<'a> {
    type Target = Vec<Box<dyn Hittable + 'a>>;

    fn deref(&self) -> &Self::Target {
        &self.objects
    }
}

impl<'a> std::ops::DerefMut for HittableList<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.objects
    }
}

impl<'a> HittableList<'a> {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }
}
