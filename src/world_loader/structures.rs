use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Vector = (f32, f32, f32);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub image: Image,
    pub camera: Camera,
    pub world: World,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Image {
    pub height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Camera {
    ThinLens {
        origin: Vector,
        look_at: Vector,
        up_vector: Vector,
        aspect_ratio: AspectRatio,
        aperture: f32,
        vertical_fov: f32,
        #[serde(default)]
        focus_distance: f32,
    },
    Isomorphic {
        origin: Vector,
        look_at: Vector,
        up_vector: Vector,
        aspect_ratio: AspectRatio,
        vertical_fov: f32,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AspectRatio {
    Float(f32),
    Fraction(u32, u32),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct World {
    pub background_color: Color,
    pub materials: HashMap<String, Material>,
    pub objects: Vec<Object>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Material {
    Lambertian {
        albedo: Color,
    },
    Metal {
        albedo: Color,
        fuzziness: f32,
    },
    Dielectric {
        attenuation: Color,
        refraction_index: f32,
    },
    Emissive {
        color: Color,
        intensity: f32,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Color {
    Rgb(f32, f32, f32),
    Hex(u32),
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
    Black,
    White,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Object {
    Sphere {
        center: Vector,
        radius: f32,
        material: String,
    },
    Triangle {
        vertex: Vector,
        edges: (Vector, Vector),
        material: String,
    },
    Quad {
        vertex: Vector,
        edges: (Vector, Vector),
        material: String,
    },
    Volumetric {
        center: Vector,
        radius: f32,
        density: f32,
        material: String,
    },
}
