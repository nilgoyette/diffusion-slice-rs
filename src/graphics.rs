use std::path::PathBuf;

use {client::Client, pipeline::Pipelines, resources::Resources};

#[macro_use]
mod macros;

mod client;
mod pipeline;
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
    pipelines: Pipelines,
}

impl Context {
    pub async fn new(inputs: UserInputs) -> Self {
        let client = Client::new(&inputs).await;
        let res = Resources::new(&inputs.src_img, &client);

        Self {
            pipelines: Pipelines::new(&res, &client.device),
            res,
            client,
        }
    }
}
