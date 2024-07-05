use std::{
    path::{Path, PathBuf},
    time::Instant,
};

use clap::Parser;
use glam::{uvec2, UVec2};

mod file;
mod graphics;

type Image = image::RgbaImage;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input NiFTI image
    pub input_image: PathBuf,

    /// Output png image
    pub output: PathBuf,

    /// Width and height of the output 2D image
    #[arg(num_args(2), long, default_values = ["800", "600"])]
    pub output_size: Vec<u32>,
}

pub struct UserInputs {
    pub src_img: Image,
    pub dst_img_size: UVec2,
    pub dst_img_path: PathBuf,
}

fn main() {
    let start = Instant::now();

    let args = Args::parse();
    println!("{args:?}");

    init_logger();

    // TODO Make this parameterable
    // TODO Validate user inputs
    let inputs = UserInputs {
        src_img: file::load_image(Path::new("sunshine.jpg")),
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
