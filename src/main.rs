use std::{path::PathBuf, time::Instant};

use clap::Parser;
use file::read_3d_image;
use glam::{uvec2, UVec2};
use ndarray::{s, Array2, ShapeBuilder};

mod file;
mod graphics;

type Image = image::RgbaImage;
type ImageSlice = Array2<u8>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input NIfTI image
    pub input_image: PathBuf,

    /// Output png image
    pub output: PathBuf,

    /// Width and height of the output 2D image
    #[arg(num_args(2), long, default_values = ["800", "600"])]
    pub output_size: Vec<u32>,
}

pub struct UserInputs {
    pub src_img: ImageSlice,
    pub dst_img_size: UVec2,
    pub dst_img_path: PathBuf,
}

fn main() {
    let start = Instant::now();
    init_logger();

    let args = Args::parse();
    println!("{args:?}");

    let (_nifti_header, data) = read_3d_image::<_, f32>(args.input_image);

    // Let's take a "random" slice for now
    let slice = data.slice(s![.., 90, ..]).to_owned();

    // Rescale whatever we've got to u8
    let image_min = data.fold(f32::MAX, |acc, &v| f32::min(acc, v));
    let image_max = data.fold(f32::MIN, |acc, &v| f32::max(acc, v));
    let image_range = image_max - image_min;
    let type_min = 0.0;
    let type_max = 255.0;
    let type_range = type_max - type_min;
    let slice: Array2<u8> =
        slice.mapv(|v| ((v - image_min) * (type_range / image_range) + type_min) as u8);

    // A NIfTI image is stored in 'f' order, but it doesn't know that much itself so ndarray thinks
    // it's the standard 'c' order. Because of this, we convert it to 'f' order, which actually
    // converts it to 'c' order. Voilà.
    let mut src_img = Array2::zeros(slice.dim().f());
    src_img.assign(&slice);

    // TODO Validate user inputs
    let inputs = UserInputs {
        src_img,
        dst_img_size: uvec2(args.output_size[0], args.output_size[1]),
        dst_img_path: PathBuf::from("output.png"),
    };
    let image = graphics::run_full_pipeline(&inputs);

    file::save_image(image, &inputs.dst_img_path);

    log::info!("Program duration: {:?}", start.elapsed());
}

fn init_logger() {
    std::env::set_var("RUST_LOG", "info,wgpu=error");
    env_logger::init();
}
