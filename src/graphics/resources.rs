use std::collections::HashMap;

use wgpu::{BindGroupLayout, Buffer};

use super::Client;

pub use texture::{Texture, COLOR_FORMAT};

pub mod bind {
    pub mod group;
    pub(super) mod layout;
}
mod buffer;
mod texture;
pub mod vertex;

type BindLayouts = HashMap<String, BindGroupLayout>;

pub struct Resources {
    pub bind_layouts: BindLayouts,

    pub multisampled_texture: Texture,
    pub target_texture: Texture,

    pub image_vertex_buffer: Buffer,
    pub transfer_buffer: Buffer,
}

impl Resources {
    pub fn new(client: &Client) -> Self {
        let device = &client.device;

        let target_texture = Texture::new_target(client);

        Self {
            bind_layouts: HashMap::from([
                ("Source".to_string(), bind::layout::source(device)),
                ("Transform".to_string(), bind::layout::transform(device)),
            ]),
            image_vertex_buffer: buffer::init_image_vertex_buffer(client),
            transfer_buffer: buffer::create_transfer_buffer(&target_texture, &client.device),

            multisampled_texture: Texture::new_multisampled(client),
            target_texture,
        }
    }
}
