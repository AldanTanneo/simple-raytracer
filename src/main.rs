mod bounding_boxes;
mod camera;
mod command_line_interface;
mod fast_random;
mod hittable;
mod materials;
mod ray;
mod vec3;
mod world_loader;

use std::f64::consts::TAU;

use anyhow::{Context, Result};
use clap::Clap;
use image::{ImageBuffer, Rgb};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressFinish, ProgressStyle};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use bounding_boxes::BoundingVolumeHierarchy;
use fast_random::SplitMix64;
pub use materials::{
    dielectric::Dielectric, emissive::Emissive, lambertian::Lambertian, metal::Metal, ScatterResult,
};
use vec3::color::Colour;

use crate::command_line_interface::Opts;

pub type FastRng = SplitMix64;

const M1: u32 = 1597334677u32;
const M2: u32 = 3812015801u32;
const M3: u32 = 2741598923u32;
const M4: f64 = 1.0 / 0xffffffffu32 as f64;

/// Quick hasher function to avoid an expensive call to the rng
#[inline(always)]
pub fn hash_fast(mut x: u32, mut y: u32, mut z: u32) -> f64 {
    x *= M1;
    y *= M2;
    z *= M3;
    let n: u32 = (x ^ y ^ z) * M1;
    n as f64 * M4
}

fn main() -> Result<()> {
    let opts: Opts = Clap::parse();

    if let Opts::Example = opts {
        command_line_interface::example_config_file();
        return Ok(());
    }

    let mut global_rng = FastRng::new(rand::random::<std::num::NonZeroU64>().into());
    let (config, out_file, display_tree) = opts.parse(&mut global_rng)?;

    println!(
        "Successfully loaded scene with {} objects and {} materials",
        config.world.objects.len(),
        config.world.materials.len()
    );

    let image_height = config.image.height;
    let image_width = image_height * config.aspect_ratio();
    let samples_per_pixel = config.image.samples_per_pixel;
    let max_depth = config.image.max_depth;
    let background_color = (&config.world.background_color).into();

    let camera = config.camera();
    let materials = config.materials();
    let hittables = config.world(&materials)?;
    let world = BoundingVolumeHierarchy::build(&hittables)
        .with_context(|| "Error building the BVH tree")?;

    let (depth, nodes) = world.depth_and_num_nodes();
    println!(
        "Successfully built BVH tree with {} nodes, depth: {}",
        nodes, depth
    );
    if display_tree {
        println!("{}", world);
    }

    let total_pixels = image_width as u64 * image_height as u64;
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

            let rng: &mut FastRng = global_rng.as_mut();

            (0..samples_per_pixel)
                .map(|k| {
                    let u = (i as f64 + hash_fast(i, j, k)) / (image_width - 1) as f64;
                    let v = (j as f64 + hash_fast(i, k, j)) / (image_height - 1) as f64;

                    camera
                        .get_ray(u, v, hash_fast(j, i, k), TAU * hash_fast(j, k, i))
                        .colour(&world, rng, max_depth, background_color)
                })
                .fold(Colour::BLACK, |a, b| a + b)
                .as_bytes(samples_per_pixel)
        })
        .collect();

    let img: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_vec(image_width, image_height, buffer)
            .ok_or_else(|| anyhow::anyhow!("Could not create image buffer: size mismatch"))?;

    pb.println(format!(
        "Scene rendered in {} seconds.\nSaving as {}...",
        pb.elapsed().as_secs(),
        out_file.display(),
    ));

    img.save(out_file)?;

    pb.println("Successfully saved image.");

    Ok(())
}
