use std::{path::PathBuf, time::Instant};

use clap::Parser;
use file::read_3d_image;
use glam::{uvec2, vec3, UVec2, Vec3};
use nalgebra::Vector3;
use slicer::{ImageSlice, Slicer, View};
use trk_io::Reader;

mod file;
mod graphics;
mod slicer;

type Image = image::RgbaImage;
type Polyline = Vec<Vec3>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input NIfTI image
    pub input_image: PathBuf,

    /// Output folder to save all png
    #[arg(short, long)]
    pub fibers: Option<PathBuf>,

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
    pub streamlines: Vec<Polyline>,
    pub dst_img_size: UVec2,
}

fn main() {
    let start = Instant::now();
    init_logger();

    let args = Args::parse();
    let (nifti_header, data) = read_3d_image::<_, f32>(args.input_image);

    let streamlines: Vec<Polyline> = match args.fibers {
        Some(path) => {
            let spacing = Vector3::new(
                nifti_header.pixdim[1],
                nifti_header.pixdim[2],
                nifti_header.pixdim[3],
            );
            let mut reader = Reader::new(path).unwrap().to_voxel_space(spacing);
            reader
                .streamlines()
                .iter()
                .map(|streamline| streamline.iter().map(|p| vec3(p.x, p.y, p.z)).collect())
                .collect()
        }
        None => vec![],
    };
    let slicer = Slicer::from_3d(nifti_header, data, 3, &args.views, (0.3, 0.7));

    let inputs = ContextInputs {
        streamlines,
        dst_img_size: uvec2(args.output_size[0], args.output_size[1]),
    };
    let mut graphics = graphics::Context::new(&inputs);

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
