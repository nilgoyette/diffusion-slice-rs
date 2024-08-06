use std::collections::HashMap;

use glam::Mat4;
use trk_io::Reader;
use wgpu::{BindGroupLayout, Buffer};

use crate::graphics::Client;
use fibers::FiberResources;

pub use texture::{Texture, COLOR_FORMAT, DEPTH_FORMAT};

pub mod bind {
    pub mod group;
    pub(super) mod layout;
}
mod buffer;
mod fibers;
mod texture;
pub mod vertex;

type BindLayouts = HashMap<String, BindGroupLayout>;

pub struct Resources {
    pub bind_layouts: BindLayouts,

    pub multisampled_texture: Texture,
    pub depth_texture: Texture,
    pub target_texture: Texture,

    pub image_vertices: Buffer,

    pub transfer_buffer: Buffer,
    pub fibers: Option<FiberResources>,

    pub transform: Buffer,
}

impl Resources {
    pub fn new(fibers: Option<Reader>, client: &Client) -> Self {
        let device = &client.device;
        let target_texture = Texture::new_target(client);

        let mut bind_layouts = vec![("Source".to_string(), bind::layout::source(device))];
        if fibers.is_some() {
            bind_layouts.push(("Transform".to_string(), bind::layout::transform(device)));
        }
        Self {
            bind_layouts: bind_layouts.into_iter().collect(),
            image_vertices: buffer::init_image_vertex_buffer(device),
            transfer_buffer: buffer::create_transfer_buffer(&target_texture, device),

            fibers: fibers.map(|fibers| FiberResources::new(fibers, device)),

            multisampled_texture: Texture::new_multisampled(client),
            depth_texture: Texture::new_depth(client),
            target_texture,

            transform: buffer::create_transform(Mat4::IDENTITY, device),
        }
    }
}
