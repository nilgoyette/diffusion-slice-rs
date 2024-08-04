use bytemuck::Pod;
use glam::{vec2, Mat4, UVec2, Vec2};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages, Device,
};

use crate::graphics::{
    resources::{vertex::ImageVertex, Texture},
    Context,
};

pub fn init_image_vertex_buffer(device: &Device) -> Buffer {
    device.create_buffer_init(&BufferInitDescriptor {
        label: label!("ImageVertexBuffer"),
        contents: bytemuck::cast_slice(&quad_vertices(Vec2::ZERO)),
        usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
    })
}

pub fn init_vertices<V: Pod>(name: &str, vertices: &[V], device: &Device) -> Buffer {
    device.create_buffer_init(&BufferInitDescriptor {
        label: label!("{name}VertexBuffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: BufferUsages::VERTEX,
    })
}

pub fn init_indices(name: &str, indices: &[u32], device: &Device) -> Buffer {
    device.create_buffer_init(&BufferInitDescriptor {
        label: label!("{name}IndexBuffer"),
        contents: bytemuck::cast_slice(&indices),
        usage: BufferUsages::INDEX,
    })
}

impl Context {
    pub fn set_image_vertices(&self, size: UVec2) {
        let vertices = quad_vertices(uv_offset(size, self.client.img_size));
        let bytes = bytemuck::cast_slice(&vertices);

        self.client
            .command_queue
            .write_buffer(&self.res.image_vertices, 0, bytes);
    }

    pub fn set_transform(&self, transform: Mat4) {
        let transform = &[transform];
        let bytes = bytemuck::cast_slice(transform);

        self.client
            .command_queue
            .write_buffer(&self.res.transform, 0, bytes);
    }
}

/// Calculates the UV offset needed to maintain the source image's aspect ratio
fn uv_offset(src_size: UVec2, dst_size: UVec2) -> Vec2 {
    let (src_size, dst_size) = (src_size.as_vec2(), dst_size.as_vec2());

    let ratio = dst_size / src_size;

    if ratio.x > ratio.y {
        let img_w = src_size.x * ratio.y;
        vec2((dst_size.x - img_w) / (2. * img_w), 0.)
    } else {
        let img_h = src_size.y * ratio.x;
        vec2(0., (dst_size.y - img_h) / (2. * img_h))
    }
}

fn quad_vertices(uv_offset: Vec2) -> [ImageVertex; 6] {
    let (du, dv) = (uv_offset.x, uv_offset.y);

    let vertex = |x, y, u, v| ImageVertex {
        canon: vec2(x, y),
        uv: vec2(u, v),
    };
    let (nu, pu) = (0. - du, 1. + du);
    let (nv, pv) = (0. - dv, 1. + dv);
    [
        vertex(1., 1., nu, 1. + dv),
        vertex(1., -1., nu, nv),
        vertex(-1., -1., pu, nv),
        vertex(1., 1., nu, pv),
        vertex(-1., -1., pu, nv),
        vertex(-1., 1., pu, pv),
    ]
}

pub fn create_transfer_buffer(texture: &Texture, device: &Device) -> Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: label!("TransferBuffer"),
        size: texture.bytes_stride as u64 * texture.inner.size().height as u64,
        usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

pub fn create_transform(transform: Mat4, device: &Device) -> Buffer {
    device.create_buffer_init(&BufferInitDescriptor {
        label: label!("TransformUniformBuffer"),
        contents: bytemuck::cast_slice(&[transform]),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    })
}
