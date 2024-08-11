use std::{path::PathBuf, time::Instant};

use clap::Parser;
use glam::{uvec2, UVec2};
use trk_io::Reader;

use file::{fibers_reader, read_3d_image};
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

    /// Use a white background instead of black
    #[arg(short, long, default_value = "false")]
    pub white: bool,

    /// Output folder to save all png
    #[arg(short, long)]
    pub fibers: Option<PathBuf>,

    /// How many streamlines are batched per buffer
    #[arg(short, long, default_value = "50000")]
    pub batch_size: usize,

    /// Output folder to save all png
    pub output: PathBuf,

    /// Width and height of the output 2D image
    #[arg(num_args(2), long, default_values = ["800", "600"])]
    pub output_size: Vec<u32>,

    /// Which view(s) to use tp capture the image(s)
    #[arg(num_args(1..7), long, default_values = ["superior", "posterior", "left"])]
    pub views: Vec<View>,
}

pub struct ContextInputs {
    pub fibers_reader: Option<Reader>,
    pub dst_img_size: UVec2,
    pub streamline_batch_size: usize,
    pub white_mode: bool,
}

fn main() {
    let start = Instant::now();
    init_logger();

    let args = Args::parse();

    let (nifti_header, data) = read_3d_image::<_, f32>(args.input_image);
    let fibers_reader = args.fibers.map(|path| fibers_reader(path, &nifti_header));

    let slicer = Slicer::from_3d(nifti_header, data, 3, &args.views, (0.3, 0.7));

    let inputs = ContextInputs {
        fibers_reader,
        dst_img_size: uvec2(args.output_size[0], args.output_size[1]),
        streamline_batch_size: args.batch_size,
        white_mode: args.white,
    };
    let mut graphics = graphics::Context::new(inputs);

    for slice in slicer.slices {
        let image = graphics.process_slice(&slice.data);

        // TODO Support a prefix, like "prefix{}_{}.png"
        let mut write_to = args.output.clone();
        write_to.push(format!("{}_{}.png", slice.view.name(), slice.index));
        file::save_image(image, &write_to);
    }

    log::info!("Program duration: {:?}", start.elapsed());
}

fn init_logger() {
    std::env::set_var("RUST_LOG", "info,wgpu=error");
    env_logger::init();
}
