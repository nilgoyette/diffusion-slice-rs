use glam::UVec2;
use wgpu::Buffer;

use super::{Client, ImageSlice};

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

    pub multisampled_texture: Texture,
    pub target_texture: Texture,

    pub image_vertex_buffer: Buffer,
    pub transfer_buffer: Buffer,
}

impl Resources {
    pub fn new(image: &ImageSlice, client: &Client) -> Self {
        let target_texture = Texture::new_target(client);
        let source_texture = Texture::new_source(image, client);

        let img_size = UVec2::new(image.dim().0 as u32, image.dim().1 as u32);

        Self {
            binding: Binding::new(&source_texture, &client.device),

            image_vertex_buffer: buffer::create_image_vertex_buffer(img_size, client),
            transfer_buffer: buffer::create_transfer_buffer(&target_texture, &client.device),

            multisampled_texture: Texture::new_multisampled(client),
            target_texture,
        }
    }
}
