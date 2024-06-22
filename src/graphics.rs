use std::path::PathBuf;

use wgpu::{Device, Queue};

use client::Client;
use resources::*;

#[macro_use]
mod macros;

mod client;
mod resources;
mod workload;

const MULTISAMPLE_COUNT: u32 = 4;

pub type Image = image::RgbaImage;

pub struct UserInputs {
    pub src_img: Image,
    pub dst_img_size: (u32, u32),
    pub dst_img_path: PathBuf,
}

pub struct Context {
    client: Client,
    res: Resources,
}

impl Context {
    pub async fn new(inputs: UserInputs) -> Self {
        let client = Client::new(&inputs).await;

        Self {
            res: Resources::new(&inputs.src_img, &client),
            client,
        }
    }
}
