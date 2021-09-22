use crate::hittable::Hittable;
use crate::materials::ScatterResult;
use crate::vec3::color::Colour;
use crate::vec3::{Point3, Vec3};
use crate::FastRng;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub const fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + self.direction * t
    }

    pub fn colour<H: Hittable>(
        self,
        world: &H,
        rng: &mut FastRng,
        max_depth: u32,
        background_color: Colour,
    ) -> Colour {
        let mut ray = self;
        let mut i = 0;
        let mut attenuation = Colour::WHITE;
        while let Some(hit_record) = world.hit(&ray, 0.001, f32::INFINITY, rng) {
            match hit_record.material.scatter(&ray, &hit_record, rng) {
                ScatterResult::Ray(scattered_ray) => {
                    attenuation *= scattered_ray.attenuation;
                    ray = scattered_ray.ray;
                }
                ScatterResult::Emissive(color) => {
                    return attenuation * color;
                }
                ScatterResult::Absorbed => break,
            }
            i += 1;
            if i == max_depth {
                break;
            }
        }
        background_color * attenuation
    }
}

#[derive(Clone, Debug)]
pub struct ScatteredRay {
    pub ray: Ray,
    pub attenuation: Colour,
}

impl ScatteredRay {
    pub fn new(ray: Ray, attenuation: Colour) -> Self {
        Self { ray, attenuation }
    }
}
