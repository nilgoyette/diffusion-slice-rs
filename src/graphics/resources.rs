use super::*;

use buffer::ImageTransferBuffer;
use texture::*;

mod buffer;
mod texture;

pub struct Resources {
    pub target_texture: Texture,
    pub transfer_buffer: ImageTransferBuffer,
}

impl Resources {
    pub fn new(client: &Client) -> Self {
        let target_texture = Texture::new_target(client);

        Self {
            transfer_buffer: ImageTransferBuffer::new(&target_texture, client),
            target_texture,
        }
    }
}
