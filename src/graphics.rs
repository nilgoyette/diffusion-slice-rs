use crate::{slicer::Slice, ContextInputs, Image};
use {client::Client, parameters::Parameters, pipeline::Pipelines, resources::Resources};

#[macro_use]
mod macros;

mod client;
mod parameters;
mod pipeline;
mod resources;
mod workload;

pub struct Context {
    client: Client,
    parameters: Parameters,
    res: Resources,
    pipelines: Pipelines,
}

impl Context {
    pub fn new(inputs: ContextInputs) -> Self {
        let client = pollster::block_on(Client::new(&inputs));
        let parameters = Parameters::new(&inputs);
        let res = Resources::new(inputs.fibers_reader, &client);

        Self {
            pipelines: Pipelines::new(&res, &client),
            client,
            parameters,
            res,
        }
    }

    pub fn process_slice(&mut self, slice: &Slice) -> Image {
        self.execute_workloads(slice)
    }
}
