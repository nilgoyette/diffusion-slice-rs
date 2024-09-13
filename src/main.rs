use std::time::Instant;

use clap::Parser;

use file::read_3d_image;
use inputs::{Args, ContextInputs};
use slicer::{ImageSlice, Slicer};

mod file;
mod graphics;
mod inputs;
mod slicer;

type Image = image::RgbaImage;

fn main() {
    let start = Instant::now();

    init_logger();

    let args = Args::parse();
    let (nifti_header, data) = read_3d_image::<_, f32>(&args.input_image);

    let inputs = ContextInputs::new(&args, &nifti_header);
    let mut graphics = graphics::Context::new(inputs);

    let slicer = Slicer::from_3d(nifti_header, data, 3, &args.views, (0.3, 0.7));

    for slice in slicer.slices {
        let image = graphics.process_slice(&slice);

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
