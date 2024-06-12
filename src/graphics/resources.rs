use super::*;

use texture::*;

mod texture;

pub struct Resources {
    pub dst_texture: TextureData,
}

impl Resources {
    pub fn new(client: &Client) -> Self {
        Self {
            dst_texture: TextureData::new_dst(client),
        }
    }
}
