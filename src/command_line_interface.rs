use std::{fs::File, path::PathBuf};

use anyhow::{ensure, Context, Result};
use clap::{AppSettings, Clap, ValueHint};
use rand::Rng;
use ron::ser::{to_writer_pretty, PrettyConfig};

use crate::world_loader::Config;
use crate::FastRng;

/// A CPU-based raytracer
#[derive(Clap)]
#[clap(version = "1.0", author = "César Sagaert <sagaert.cesar@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(setting = AppSettings::ArgRequiredElseHelp)]
pub enum Opts {
    /// Render a scene according to a configuration file
    Render {
        /// A .ron configuration file
        #[clap(parse(from_os_str), value_hint = ValueHint::FilePath)]
        config: PathBuf,
        /// An output image file. Can be a .jpeg or .png
        #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath)]
        output: Option<PathBuf>,
        /// Displays the BVH tree
        #[clap(short, long)]
        tree: bool,
    },
    /// Renders a semi-randomly generated scene
    Random {
        /// An output image or config file. Can be a .jpeg or a .png
        #[clap(short, long, default_value = "random_scene.png", parse(from_os_str), value_hint = ValueHint::FilePath)]
        output: PathBuf,
        /// If specified, saves the config .ron file to the given file name
        #[clap(long, parse(from_os_str), value_hint = ValueHint::FilePath)]
        save: Option<PathBuf>,
        /// Random seed, to generate repeatable results
        #[clap(long)]
        seed: Option<u64>,
        /// Displays the BVH tree
        #[clap(short, long)]
        tree: bool,
    },
    /// Display an example configuration file
    Example,
}

#[derive(Clone, Debug)]
pub struct EarlyReturn;

impl std::fmt::Display for EarlyReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Early Return")
    }
}

impl Opts {
    pub fn parse(self, rng: &mut impl Rng) -> Result<(Config, PathBuf, bool)> {
        match self {
            Self::Render {
                config,
                output,
                tree,
            } => {
                ensure!(
                    config.extension().map(|s| s == "ron").unwrap_or_default(),
                    "Expecting a .ron config file."
                );
                let parsed_config =
                    Config::parse(&config).with_context(|| "Error parsing the config file")?;
                let output_file = output.unwrap_or_else(|| config.with_extension("png"));
                Ok((parsed_config, output_file, tree))
            }
            Self::Random {
                output,
                save,
                tree,
                seed,
            } => {
                let config = if let Some(seed) = seed {
                    Config::random_scene(&mut FastRng::new(seed))
                } else {
                    Config::random_scene(rng)
                };
                if let Some(file) = save {
                    let out_file = File::create(&file)
                        .with_context(|| "Error creating the random config file")?;
                    println!("Saving randomly generated scene to `{}`", file.display());
                    to_writer_pretty(out_file, &config, PrettyConfig::new())?;
                }
                println!(
                    "Rendering a random scene (image size: {}x{}, {}spp).",
                    config.image.height,
                    config.image.height * config.aspect_ratio(),
                    config.image.samples_per_pixel
                );
                Ok((config, output, tree))
            }
            Self::Example => {
                panic!("This case should have been handled earlier.")
            }
        }
    }
}

const EXAMPLE_FILE: &str = r#"/*
=== SAMPLE CONFIG FILE === (does not render anything pretty)
Available constant colors: Red, Yellow, Green, Cyan, Blue, Magenta, Black, White
Other color formats: Rgb(float, float, float), Hex(int)
*/
Config(
    image: (
        height: 400, // image height in pixels
        samples_per_pixel: 200, // the number of rays cast per pixel
        max_depth: 50, // the maximum bounce depth
    ),
    camera: ThinLens( // the camera can also be isomorphic,
                      // in which case aperture and focus distance are not needed.
        origin: (13, 2, 3),
        look_at: (0, 0, 0),
        up_vector: (0, 1, 0), // decides of the camera orientation
        aspect_ratio: (16, 9), // can be a tuple or a float
        aperture: 0.1,
        vertical_fov: 30,
        focus_distance: 10, // optional
    ),
    world: (
        background_color: Rgb(0.2, 0.2, 0.2),
        materials: {
            "diffuse": Lambertian(
                albedo: Rgb(0.5, 0.1, 1.0),
            ),
            "metal": Metal(
                albedo: Hex(0x15A2FF),
                fuzziness: 0.2,
            ),
            "glass": Dielectric(
                attenuation: Red,
                refraction_index: 1.5,
            ),
            "emissive": Emissive(
                color: Yellow,
                intensity: 2.0,
            ),
            "plastic": Plastic(
                albedo: Blue,
                roughness: 0.5,
            )
        },
        objects: [
            Sphere(
                center: (-26, -4, -6),
                radius: 5.2,
                material: "emissive",
            ),
            Triangle(
                vertex: (1, 0, 0),
                edges: (
                    (0, 1, 0),
                    (0.1, 0, 1),
                ),
                material: "metal",
            ),
            Quad(
                vertex: (2, 1, 3),
                edges: (
                    (0, 4, 3),
                    (0, 1, 5),
                ),
                material: "glass",
            ),
            Volumetric(
                center: (0, 0, 2),
                radius: 1,
                density: 0.9,
                material: "diffuse",
            )
        ]
    )
)"#;

pub fn example_config_file() {
    println!("{}", EXAMPLE_FILE);
}
