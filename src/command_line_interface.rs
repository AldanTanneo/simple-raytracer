use clap::{AppSettings, Clap};

/// A CPU-based raytracer
#[derive(Clap)]
#[clap(version = "1.0", author = "César Sagaert <sagaert.cesar@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(setting = AppSettings::ArgRequiredElseHelp)]
pub struct Opts {
    /// A .ron configuration file
    #[clap(conflicts_with = "example")]
    pub scene: Option<String>,
    /// An output image file
    #[clap(short, long, conflicts_with = "example")]
    pub output: Option<String>,
    /// Displays the BVH tree
    #[clap(short, long, conflicts_with = "example")]
    pub tree: bool,
    /// Displays an example config file. Cannot be used with other arguments.
    #[clap(long, conflicts_with = "random")]
    pub example: bool,
    /// Renders a randomly generated scene
    #[clap(long, conflicts_with = "scene")]
    pub random: bool,
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
