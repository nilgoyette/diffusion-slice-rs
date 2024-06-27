use std::{
    path::{Path, PathBuf},
    time::Instant,
};

mod file;
mod graphics;

type Image = image::RgbaImage;

pub struct UserInputs {
    pub src_img: Image,
    pub dst_img_size: (u32, u32),
    pub dst_img_path: PathBuf,
}

fn main() {
    let start = Instant::now();

    init_logger();

    // TODO Make this parameterable
    // TODO Validate user inputs
    let inputs = UserInputs {
        src_img: file::load_image(Path::new("sunshine.jpg")),
        dst_img_size: (800, 600),
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
