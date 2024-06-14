use wgpu::Buffer;

use super::*;

pub use texture::*;

mod buffer;
mod texture;

pub struct Resources {
    pub source_texture: Texture,
    pub target_texture: Texture,
    pub transfer_buffer: Buffer,
}

impl Resources {
    pub fn new(image: &Image, client: &Client) -> Self {
        let target_texture = Texture::new_target(client);

        Self {
            transfer_buffer: buffer::create_image_transfer_buffer(&target_texture, &client.device),
            source_texture: Texture::new_source(image, client),
            target_texture,
        }
    }
}
