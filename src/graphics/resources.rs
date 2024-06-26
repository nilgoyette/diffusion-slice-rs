use wgpu::Buffer;

use super::{Client, Image};

pub use texture::{Texture, COLOR_FORMAT};

mod buffer;
mod texture;
pub mod vertex;

pub struct Resources {
    #[allow(unused)]
    pub source_texture: Texture,
    pub multisampled_texture: Texture,
    pub target_texture: Texture,

    pub image_vertex_buffer: Buffer,
    pub transfer_buffer: Buffer,
}

impl Resources {
    pub fn new(image: &Image, client: &Client) -> Self {
        let target_texture = Texture::new_target(client);

        Self {
            image_vertex_buffer: buffer::create_image_vertex_buffer(&client.device),
            transfer_buffer: buffer::create_transfer_buffer(&target_texture, &client.device),

            source_texture: Texture::new_source(image, client),
            multisampled_texture: Texture::new_multisampled(client),
            target_texture,
        }
    }
}
