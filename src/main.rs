use std::{path::PathBuf, time::Instant};

use clap::Parser;
use glam::{uvec2, uvec3, UVec2, UVec3};
use nalgebra::Vector3;
use nifti::NiftiHeader;
use trk_io::Reader;

use file::{fibers_reader, read_3d_image};
use graphics::Coloring;
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
    #[arg(short, long, default_value = "50000", requires("fibers"))]
    pub batch_size: usize,

    /// Color mode for the fibers
    #[arg(long, default_value = "local", requires("fibers"))]
    pub coloring: ColoringInput,

    /// RGB Color used by the uniform coloring mode
    #[arg(
        num_args(3),
        long,
        required_if_eq("coloring", "uniform"),
        value_names = &["R", "G", "B"]
    )]
    pub rgb: Vec<u32>,

    /// Output folder to save all png
    pub output: PathBuf,

    /// Width and height of the output 2D image
    #[arg(
        num_args(2), 
        long, 
        default_values = ["800", "600"], 
        value_names = &["WIDTH", "HEIGHT"] 
    )]
    pub output_size: Vec<u32>,

    /// Which view(s) to use tp capture the image(s)
    #[arg(num_args(1..7), long, default_values = ["superior", "posterior", "left"])]
    pub views: Vec<View>,
}

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum ColoringInput {
    Local,
    Endpoint,
    Uniform,
}

pub struct ContextInputs {
    pub fibers_reader: Option<Reader>,
    pub size_3d: UVec3,
    pub dst_img_size: UVec2,
    pub streamline_batch_size: usize,
    pub white_mode: bool,
    pub coloring: Coloring,
}

fn main() {
    let start = Instant::now();
    init_logger();

    let args = Args::parse();

    let (nifti_header, data) = read_3d_image::<_, f32>(args.input_image);
    let fibers_reader = args.fibers.map(|path| fibers_reader(path, &nifti_header));

    let size_3d = get_dim(&nifti_header);
    let slicer = Slicer::from_3d(nifti_header, data, 3, &args.views, (0.3, 0.7));

    let coloring = match args.coloring {
        ColoringInput::Local => Coloring::Local,
        ColoringInput::Endpoint => Coloring::Endpoint,
        ColoringInput::Uniform => {
            Coloring::Uniform(Vector3::new(args.rgb[0], args.rgb[1], args.rgb[2]))
        }
    };

    let inputs = ContextInputs {
        fibers_reader,
        size_3d,
        dst_img_size: uvec2(args.output_size[0], args.output_size[1]),
        streamline_batch_size: args.batch_size,
        white_mode: args.white,
        coloring,
    };
    let mut graphics = graphics::Context::new(inputs);

    for slice in slicer.slices {
        let image = graphics.process_slice(&slice);

        // TODO Support a prefix, like "prefix{}_{}.png"
        let mut write_to = args.output.clone();
        write_to.push(format!("{}_{}.png", slice.view.name(), slice.index));
        file::save_image(image, &write_to);
    }

    log::info!("Program duration: {:?}", start.elapsed());
}

fn get_dim(nifti_header: &NiftiHeader) -> UVec3 {
    let dim: &[u16] = nifti_header
        .dim()
        .expect("The NIfTI must have consistent dimensions");

    if dim.len() != 3 {
        panic!("A 3D image is expected.")
    }
    uvec3(dim[0] as u32, dim[1] as u32, dim[2] as u32)
}

fn init_logger() {
    std::env::set_var("RUST_LOG", "info,wgpu=error");
    env_logger::init();
}
