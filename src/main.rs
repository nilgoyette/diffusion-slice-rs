use std::{path::PathBuf, time::Instant};

use clap::Parser;
use file::read_3d_image;
use glam::{uvec2, UVec2};
use slicer::{ImageSlice, Slicer, View};

mod file;
mod graphics;
mod slicer;

type Image = image::RgbaImage;

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

    let (nifti_header, data) = read_3d_image::<_, f32>(args.input_image);
    let slicer = Slicer::from_3d(nifti_header, data, 1, View::Anterior, (0.3, 0.7));
    for slice in slicer.slices {
        let inputs = UserInputs {
            src_img: slice.data,
            dst_img_size: uvec2(args.output_size[0], args.output_size[1]),
            dst_img_path: PathBuf::from("output.png"),
        };
        let image = graphics::run_full_pipeline(&inputs);

        // TODO Rename output image according to view and index
        file::save_image(image, &inputs.dst_img_path);
    }

    log::info!("Program duration: {:?}", start.elapsed());
}

fn init_logger() {
    std::env::set_var("RUST_LOG", "info,wgpu=error");
    env_logger::init();
}
