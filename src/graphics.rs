use crate::{ContextInputs, Image, ImageSlice};
use {client::Client, pipeline::Pipelines, resources::Resources};

#[macro_use]
mod macros;

mod client;
mod pipeline;
mod resources;
mod workload;

pub struct Context {
    client: Client,
    res: Resources,
    pipelines: Pipelines,
}

impl Context {
    pub fn new(inputs: ContextInputs) -> Self {
        let client = pollster::block_on(Client::new(&inputs));
        let res = Resources::new(inputs.fibers_reader, &client);

        Self {
            pipelines: Pipelines::new(&res, &client),
            res,
            client,
        }
    }

    pub fn process_slice(&mut self, image: &ImageSlice) -> Image {
        self.execute_workloads(image)
    }
}
