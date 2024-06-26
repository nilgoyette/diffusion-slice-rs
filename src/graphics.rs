use crate::{Image, UserInputs};
use {client::Client, pipeline::Pipelines, resources::Resources};

#[macro_use]
mod macros;

mod client;
mod pipeline;
mod resources;
mod workload;

struct Context {
    client: Client,
    res: Resources,
    pipelines: Pipelines,
}

impl Context {
    async fn new(inputs: &UserInputs) -> Self {
        let client = Client::new(inputs).await;
        let res = Resources::new(&inputs.src_img, &client);

        Self {
            pipelines: Pipelines::new(&res, &client),
            res,
            client,
        }
    }
}

pub fn run_full_pipeline(inputs: &UserInputs) -> Image {
    pollster::block_on(Context::new(&inputs)).execute_workloads()
}
