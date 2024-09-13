use std::collections::HashMap;

use glam::{vec2, Mat3, Mat4};
use trk_io::Reader;
use wgpu::{BindGroupLayout, Buffer};

use crate::graphics::Client;
use fibers::FiberBatch;
use vertex::ImageVertex;

pub use {
    coloring::Coloring,
    texture::{Texture, COLOR_FORMAT, DEPTH_FORMAT},
};

pub mod bind {
    pub mod group;
    pub(super) mod layout;
}
mod buffer;
mod coloring;
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
    pub fibers: Vec<FiberBatch>,

    pub transform: Buffer,
}

impl Resources {
    pub fn new(fibers: Option<Reader>, client: &Client) -> Self {
        let device = &client.device;
        let target_texture = Texture::new_target(client);

        let mut bind_layouts = vec![("Source".to_string(), bind::layout::source(device))];

        let fibers = if let Some(fibers) = fibers {
            bind_layouts.push(("Transform".to_string(), bind::layout::transform(device)));
            fibers::batches(fibers, client)
        } else {
            vec![]
        };
        Self {
            bind_layouts: bind_layouts.into_iter().collect(),

            image_vertices: buffer::init_image_vertex_buffer(device),

            transfer_buffer: buffer::create_transfer_buffer(&target_texture, device),
            fibers,

            multisampled_texture: Texture::new_multisampled(client),
            depth_texture: Texture::new_depth(client),
            target_texture,

            transform: buffer::create_transform(Mat4::IDENTITY, device),
        }
    }
}

pub fn quad_vertices(transform: Mat3) -> [ImageVertex; 6] {
    let vertex = |x, y, u, v| ImageVertex {
        canon: transform.transform_point2(vec2(x, y)),
        uv: vec2(u, v),
    };
    [
        vertex(1., 1., 0., 1.),
        vertex(1., -1., 0., 0.),
        vertex(-1., -1., 1., 0.),
        vertex(1., 1., 0., 1.),
        vertex(-1., -1., 1., 0.),
        vertex(-1., 1., 1., 1.),
    ]
}
