#![allow(clippy::borrowed_box)]

mod bounding_boxes;
mod camera;
mod command_line_interface;
mod fast_random;
mod hittable;
mod materials;
mod ray;
mod vec3;
mod world_loader;

use anyhow::Result;
use clap::Clap;
use image::{ImageBuffer, Rgb};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressFinish, ProgressStyle};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use bounding_boxes::BoundingVolumeHierarchy;
use fast_random::SplitMix64;
use hittable::Hittable;
pub use materials::{
    dielectric::Dielectric, emissive::Emissive, lambertian::Lambertian, metal::Metal, ScatterResult,
};
use ray::Ray;
use vec3::color::Color;
use world_loader::Config;

pub type FastRng = SplitMix64;

const M1: u32 = 1597334677u32;
const M2: u32 = 3812015801u32;
const M3: u32 = 2741598923u32;
const M4: f64 = 1.0 / 0xffffffffu32 as f64;

#[inline(always)]
pub fn hash_fast(mut x: u32, mut y: u32, mut z: u32) -> f64 {
    x *= M1;
    y *= M2;
    z *= M3;
    let n: u32 = (x ^ y ^ z) * M1;
    n as f64 * M4
}

fn ray_color<H: Hittable>(
    ray: &Ray,
    world: &H,
    rng: &mut FastRng,
    max_depth: u32,
    background_color: Color,
) -> Color {
    let mut ray = ray.clone();
    let mut i = 0;
    let mut attenuation = Color::WHITE;
    while let Some(hit_record) = world.hit(&ray, 0.001, f64::INFINITY, rng) {
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

fn main() -> Result<()> {
    let mut global_rng = FastRng::default();

    let mut opts = command_line_interface::Opts::parse();
    if opts.example {
        command_line_interface::example_config_file();
        return Ok(());
    }
    let config = if let Some(config_file) = &opts.scene {
        anyhow::ensure!(
            config_file.ends_with(".ron"),
            "Expecting a .ron config file."
        );
        Config::parse(config_file)?
    } else if opts.random {
        opts.scene = Some("random_scene.ron".into());
        Config::random_scene(&mut global_rng)
    } else {
        anyhow::bail!("There's nothing to render. Use --help to learn more.")
    };

    let out_file = opts.output.clone().unwrap_or_else(|| {
        format!(
            "{}.png",
            opts.scene
                .as_ref()
                .expect("unreachable")
                .trim_end_matches(".ron")
        )
    });

    println!(
        "Successfully loaded scene with {} objects and {} materials",
        config.world.objects.len(),
        config.world.materials.len()
    );

    let image_height = config.image.height;
    let image_width = image_height * config.aspect_ratio();
    let total_pixels = image_height as u64 * image_width as u64;
    let samples_per_pixel = config.image.samples_per_pixel;
    let max_depth = config.image.max_depth;
    let background_color = (&config.world.background_color).into();

    let camera = config.camera();
    let materials = config.materials();
    let hittables = config.world(&materials)?;
    let world = BoundingVolumeHierarchy::build(&hittables)?;

    let (depth, nodes) = world.depth_and_num_nodes();
    println!(
        "Successfully built BVH tree with {} nodes, depth: {}",
        nodes, depth
    );
    if opts.tree {
        println!("{}", world);
    }

    let pb = ProgressBar::new(total_pixels).with_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:50} {percent}%")
            .on_finish(ProgressFinish::AndLeave),
    );
    pb.set_draw_delta(total_pixels / 100);

    let buffer: Vec<u8> = (0..image_height * image_width)
        .into_par_iter()
        .progress_with(pb.clone())
        .flat_map(|index| {
            let j = image_height - 1 - index / image_width;
            let i = index % image_width;
            /*let mut rng = FastRng::new(j as u64);*/
            let rng: &mut FastRng = global_rng.as_mut();

            (0..samples_per_pixel)
                .map(|k| {
                    let u = (i as f64 + hash_fast(i, j, k)) / (image_width - 1) as f64;
                    let v = (j as f64 + hash_fast(i, k, j)) / (image_height - 1) as f64;
                    ray_color(
                        &camera.get_ray(
                            u,
                            v,
                            hash_fast(j, i, k),
                            std::f64::consts::TAU * hash_fast(j, k, i),
                        ),
                        &world,
                        rng,
                        max_depth,
                        background_color,
                    )
                })
                .fold(Color::BLACK, |a, b| a + b)
                .as_bytes(samples_per_pixel)
        })
        .collect();

    let img: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_vec(image_width, image_height, buffer)
            .expect("Image buffer size mismatch");

    pb.println(format!(
        "Scene rendered in {} seconds.\nSaving as {}...",
        pb.elapsed().as_secs(),
        out_file
    ));

    img.save(out_file)?;

    pb.println("Successfully saved image.");

    Ok(())
}
