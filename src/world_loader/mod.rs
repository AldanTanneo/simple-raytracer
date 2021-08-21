mod random_scene;
mod structures;

use anyhow::{anyhow, Result};
use ron::de::from_reader;
use std::collections::HashMap;
use std::fs::File;

use crate::{
    camera,
    hittable::hittable_list,
    hittable::{quad, sphere, triangle, volumetric},
    materials,
    materials::{dielectric, emissive, lambertian, metal},
    vec3::{color, Vec3},
};

pub use structures::Config;

use structures::*;

impl From<Vector> for Vec3 {
    fn from(v: Vector) -> Self {
        Self::new(v.0, v.1, v.2)
    }
}

use Color::*;

impl From<&Color> for color::Color {
    fn from(c: &Color) -> Self {
        match c {
            /* IntRgb(r, g, b) => {
                color::Color::new(*r as f64 / 255.0, *g as f64 / 255.0, *b as f64 / 255.0)
            }*/
            Rgb(red, green, blue) => color::Color::new(*red, *green, *blue),
            Hex(mut value) => {
                let blue = (value % 256) as f32 / 255.0;
                value /= 256;
                let green = (value % 256) as f32 / 255.0;
                value /= 256;
                let red = (value % 256) as f32 / 255.0;
                color::Color::new(red, green, blue)
            }
            Red => color::Color::RED,
            Green => color::Color::GREEN,
            Blue => color::Color::BLUE,
            Yellow => color::Color::YELLOW,
            Magenta => color::Color::MAGENTA,
            Cyan => color::Color::CYAN,
            Black => color::Color::BLACK,
            White => color::Color::WHITE,
        }
    }
}

impl From<&AspectRatio> for f64 {
    fn from(aspect_ratio: &AspectRatio) -> Self {
        match aspect_ratio {
            AspectRatio::Float(f) => *f,
            AspectRatio::Fraction(a, b) => *a as f64 / *b as f64,
        }
    }
}

impl std::ops::Mul<&AspectRatio> for u32 {
    type Output = u32;

    fn mul(self, rhs: &AspectRatio) -> Self::Output {
        match rhs {
            AspectRatio::Float(f) => (self as f64 * f) as u32,
            AspectRatio::Fraction(a, b) => (self * a) / b,
        }
    }
}

impl From<&Camera> for camera::Camera {
    fn from(camera: &Camera) -> Self {
        match camera {
            Camera::ThinLens {
                origin,
                look_at,
                up_vector,
                aspect_ratio,
                aperture,
                vertical_fov,
                focus_distance,
            } => {
                let origin = (*origin).into();
                let look_at = (*look_at).into();
                camera::Camera::new(
                    origin,
                    look_at,
                    (*up_vector).into(),
                    aspect_ratio.into(),
                    *aperture,
                    #[allow(clippy::float_cmp)]
                    if focus_distance != &0.0 {
                        *focus_distance
                    } else {
                        (look_at - origin).length()
                    },
                    *vertical_fov,
                )
            }
            Camera::Isomorphic {
                origin,
                look_at,
                up_vector,
                aspect_ratio,
                vertical_fov,
            } => camera::Camera::isomorphic(
                (*origin).into(),
                (*look_at).into(),
                (*up_vector).into(),
                aspect_ratio.into(),
                *vertical_fov,
            ),
        }
    }
}

impl From<&Material> for Box<dyn materials::Material> {
    fn from(material: &Material) -> Self {
        match material {
            Material::Lambertian { albedo } => Box::new(lambertian::Lambertian::new(albedo.into())),
            Material::Metal { albedo, fuzziness } => {
                Box::new(metal::Metal::new(albedo.into(), *fuzziness))
            }
            Material::Dielectric {
                attenuation,
                refraction_index,
            } => Box::new(dielectric::Dielectric::new(
                attenuation.into(),
                *refraction_index,
            )),
            Material::Emissive { color, intensity } => {
                Box::new(emissive::Emissive::new(color.into(), *intensity))
            }
        }
    }
}

impl Config {
    pub fn parse(filename: &str) -> Result<Self> {
        let file = File::open(filename)?;
        from_reader::<File, Config>(file).map_err(|e| anyhow!(e))
    }

    pub fn camera(&self) -> camera::Camera {
        (&self.camera).into()
    }

    pub fn aspect_ratio(&self) -> &AspectRatio {
        match &self.camera {
            Camera::ThinLens { aspect_ratio, .. } => aspect_ratio,
            Camera::Isomorphic { aspect_ratio, .. } => aspect_ratio,
        }
    }

    pub fn materials<'a>(&'a self) -> HashMap<&'a String, Box<dyn materials::Material + 'a>> {
        self.world
            .materials
            .iter()
            .map(|(name, mat)| (name, mat.into()))
            .collect()
    }

    pub fn world<'a>(
        &'a self,
        materials: &'a HashMap<&'a String, Box<dyn materials::Material + 'a>>,
    ) -> Result<hittable_list::HittableList<'a>> {
        let mut world = hittable_list::HittableList::new();

        for object in &self.world.objects {
            match object {
                Object::Sphere {
                    center,
                    radius,
                    material,
                } => {
                    let material = materials.get(material).ok_or_else(|| {
                        anyhow!(
                            "Could not add sphere with material \"{}\": undeclared material.",
                            material
                        )
                    })?;
                    world.push(Box::new(sphere::Sphere::new(
                        (*center).into(),
                        *radius,
                        material,
                    )))
                }
                Object::Triangle {
                    vertex,
                    edges: (a, b),
                    material,
                } => {
                    let material = materials.get(material).ok_or_else(|| {
                        anyhow!(
                            "Could not add triangle with material \"{}\": undeclared material.",
                            material
                        )
                    })?;
                    world.push(Box::new(triangle::Triangle::new(
                        (*vertex).into(),
                        ((*a).into(), (*b).into()),
                        material,
                    )));
                }
                Object::Quad {
                    vertex,
                    edges: (a, b),
                    material,
                } => {
                    let material = materials.get(material).ok_or_else(|| {
                        anyhow!(
                            "Could not add quad with material \"{}\": undeclared material.",
                            material
                        )
                    })?;
                    world.push(Box::new(quad::Quad::new(
                        (*vertex).into(),
                        ((*a).into(), (*b).into()),
                        material,
                    )));
                }
                Object::Volumetric {
                    center,
                    radius,
                    density,
                    material,
                } => {
                    let material = materials.get(material).ok_or_else(|| {
                        anyhow!(
                            "Could not add quad with material \"{}\": undeclared material.",
                            material
                        )
                    })?;
                    world.push(Box::new(volumetric::Volume::new(
                        (*center).into(),
                        *radius,
                        *density,
                        material,
                    )));
                }
            }
        }
        Ok(world)
    }
}

#[cfg(test)]
mod tests {
    use ron::from_str;
    use serde::Deserialize;

    #[derive(Clone, Debug, Deserialize)]
    struct Color {
        val: u32,
    }

    const TEST: &str = r#"
Color(
    val: 0xffffff
)
"#;

    #[test]
    fn test() {
        let test = from_str::<Color>(TEST);
        println!("{:?}", test);
    }
}