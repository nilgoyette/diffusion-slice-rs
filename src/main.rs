mod graphics;

fn main() {
    #[cfg(debug_assertions)]
    init_logger();

    // TODO: Make this parameterable
    //       Validate user inputs
    let inputs = graphics::UserInputs {
        dst_img_size: (800, 600),
        dst_img_path: std::path::PathBuf::from("output.png"),
    };
    let ctx = pollster::block_on(graphics::Context::new(inputs));
    ctx.execute_workloads();
}

fn init_logger() {
    std::env::set_var("RUST_LOG", "error");
    env_logger::init();
}
