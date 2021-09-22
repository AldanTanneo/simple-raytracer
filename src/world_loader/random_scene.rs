use rand::Rng;
use std::collections::HashMap;

use super::structures::*;
use crate::vec3::Vec3;

impl Config {
    pub fn random_scene<T: Rng>(rng: &mut T) -> Self {
        let image = Image {
            height: 405,
            samples_per_pixel: 100,
            max_depth: 20,
        };
        let camera = Camera::ThinLens {
            origin: (13.0, 2.0, 3.0),
            look_at: (0.0, 0.0, 0.0),
            up_vector: (0.0, 1.0, 0.0),
            aspect_ratio: AspectRatio::Fraction(16, 9),
            vertical_fov: 30.0,
            aperture: 0.1,
            focus_distance: 10.0,
        };
        let mut world = World {
            background_color: Color::White,
            materials: HashMap::new(),
            objects: Vec::new(),
        };

        world.materials.insert(
            String::from("ground"),
            Material::Lambertian {
                albedo: Color::Rgb(0.5, 0.5, 0.5),
            },
        );
        world.objects.push(Object::Sphere {
            center: (0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: String::from("ground"),
        });
        world.materials.insert(
            String::from("glass"),
            Material::Dielectric {
                attenuation: Color::White,
                refraction_index: 1.5,
            },
        );
        world.objects.push(Object::Sphere {
            center: (0.0, 1.0, 0.0),
            radius: 1.0,
            material: String::from("glass"),
        });
        world.materials.insert(
            String::from("brown"),
            Material::Lambertian {
                albedo: Color::Rgb(0.4, 0.2, 0.1),
            },
        );
        world.objects.push(Object::Sphere {
            center: (-4.0, 1.0, 0.0),
            radius: 1.0,
            material: String::from("brown"),
        });
        world.materials.insert(
            String::from("metal"),
            Material::Metal {
                albedo: Color::Rgb(0.7, 0.6, 0.5),
                fuzziness: 0.0,
            },
        );
        world.objects.push(Object::Sphere {
            center: (4.0, 1.0, 0.0),
            radius: 1.0,
            material: String::from("metal"),
        });

        for a in -11..11 {
            for b in -11..11 {
                let (choose_mat, a_rand, b_rand): (f64, f64, f64) = rng.gen();
                let center: Vec3 = (a as f64 + 0.9 * a_rand, 0.2, b as f64 + 0.9 * b_rand).into();

                if (center - Vec3::new(4.0, 0.2, 0.0)).length_squared() > 0.81 {
                    if choose_mat < 0.8 {
                        // diffuse
                        let albedo = Vec3::random(rng) * Vec3::random(rng);
                        let material_name = format!("diffuse.{}.{}", a, b);
                        world.materials.insert(
                            material_name.clone(),
                            Material::Lambertian {
                                albedo: Color::Rgb(
                                    albedo.x as f64,
                                    albedo.y as f64,
                                    albedo.z as f64,
                                ),
                            },
                        );
                        world.objects.push(Object::Sphere {
                            center: center.components(),
                            radius: 0.2,
                            material: material_name,
                        });
                    } else if choose_mat < 0.95 {
                        // metal
                        let albedo = Vec3::random(rng) * 0.5 + 0.5;
                        let fuzziness = rng.gen::<f64>() * 0.5;
                        let material_name = format!("metal.{}.{}", a, b);
                        world.materials.insert(
                            material_name.clone(),
                            Material::Metal {
                                albedo: Color::Rgb(
                                    albedo.x as f64,
                                    albedo.y as f64,
                                    albedo.z as f64,
                                ),
                                fuzziness,
                            },
                        );
                        world.objects.push(Object::Sphere {
                            center: center.components(),
                            radius: 0.2,
                            material: material_name,
                        });
                    } else {
                        // glass
                        world.objects.push(Object::Sphere {
                            center: center.components(),
                            radius: 0.2,
                            material: String::from("glass"),
                        });
                    }
                }
            }
        }

        Self {
            image,
            camera,
            world,
        }
    }
}

#[cfg(test)]
mod test {
    use ron::ser::{to_writer_pretty, PrettyConfig};
    use std::fs::File;

    use crate::world_loader::Config;

    #[test]
    fn test_random_scene() {
        let scene = Config::random_scene(&mut rand::thread_rng());

        let mut out_file = File::create("random_scene2.ron").expect("Could not create file");

        to_writer_pretty(&mut out_file, &scene, PrettyConfig::new())
            .expect("Could not write to file");
    }
}
