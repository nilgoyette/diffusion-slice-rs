use wgpu::Buffer;

use super::{Client, Image};

pub use {
    binding::Binding,
    texture::{Texture, COLOR_FORMAT},
};

mod binding;
mod buffer;
mod texture;
pub mod vertex;

pub struct Resources {
    pub binding: Binding,

    pub _source_texture: Texture,
    pub multisampled_texture: Texture,
    pub target_texture: Texture,

    pub image_vertex_buffer: Buffer,
    pub transfer_buffer: Buffer,
}

impl Resources {
    pub fn new(image: &Image, client: &Client) -> Self {
        let target_texture = Texture::new_target(client);
        let _source_texture = Texture::new_source(image, client);

        Self {
            binding: Binding::new(&_source_texture, &client.device),

            image_vertex_buffer: buffer::create_image_vertex_buffer(image.dimensions(), &client),
            transfer_buffer: buffer::create_transfer_buffer(&target_texture, &client.device),

            _source_texture,
            multisampled_texture: Texture::new_multisampled(client),
            target_texture,
        }
    }
}
