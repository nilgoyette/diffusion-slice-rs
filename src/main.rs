use std::{path::PathBuf, time::Instant};

mod graphics;

fn main() {
    let start = Instant::now();

    init_logger();

    // TODO Make this parameterable
    // TODO Validate user inputs
    let image = image::load_from_memory(include_bytes!("../sunshine.jpg"))
        .unwrap()
        .into_rgba8();

    let inputs = graphics::UserInputs {
        src_img: image,
        dst_img_size: (800, 600),
        dst_img_path: PathBuf::from("output.png"),
    };
    let ctx = pollster::block_on(graphics::Context::new(inputs));
    ctx.execute_workloads();

    log::info!("Program duration: {:?}", start.elapsed());
}

fn init_logger() {
    std::env::set_var("RUST_LOG", "info,wgpu=error");
    env_logger::init();
}
