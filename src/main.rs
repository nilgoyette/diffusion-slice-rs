mod graphics;

fn main() {
    let _ctx = pollster::block_on(graphics::Context::new());

    #[cfg(debug_assertions)]
    init_logger();
}

fn init_logger() {
    std::env::set_var("RUST_LOG", "error");
    env_logger::init();
}
