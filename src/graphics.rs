use wgpu::{Device, Queue};

use client::Client;
use resources::Resources;

#[macro_use]
mod macros;

mod client;
mod resources;

const MULTISAMPLE_COUNT: u32 = 8;

pub struct Context {
    #[allow(unused)] // NOTE unused for now
    client: Client,
    res: Resources,
}

impl Context {
    pub async fn new(dst_img_size: (u32, u32)) -> Self {
        let client = Client::new(dst_img_size).await;

        Self {
            res: Resources::new(&client),
            client,
        }
    }
}
