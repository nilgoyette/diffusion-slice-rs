use super::*;

use buffer::ImageTransferBuffer;
use texture::*;

mod buffer;
mod texture;

pub struct Resources {
    pub dst_texture: Texture,
    pub transfer_buffer: ImageTransferBuffer,
}

impl Resources {
    pub fn new(client: &Client) -> Self {
        let dst_texture = Texture::new_dst(client);

        Self {
            transfer_buffer: ImageTransferBuffer::new(&dst_texture, client),
            dst_texture,
        }
    }
}
