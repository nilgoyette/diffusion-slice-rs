mod graphics;

fn main() {
    let size = (800, 600); // TODO: Make this parameterable
    let ctx = pollster::block_on(graphics::Context::new(size));

    // ctx.output_image();

    #[cfg(debug_assertions)]
    init_logger();
}

fn init_logger() {
    std::env::set_var("RUST_LOG", "error");
    env_logger::init();
}
