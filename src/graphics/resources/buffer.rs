use bytemuck::Pod;
use glam::{Mat3, Mat4, Vec2};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages, Device,
};

use crate::graphics::resources::{quad_vertices, Texture};

pub fn init_image_vertex_buffer(device: &Device) -> Buffer {
    device.create_buffer_init(&BufferInitDescriptor {
        label: label!("ImageVertexBuffer"),
        contents: bytemuck::cast_slice(&quad_vertices(Vec2::ZERO, Mat3::ZERO)),
        usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
    })
}

pub fn init_vertices<V: Pod>(name: &str, vertices: &[V], device: &Device) -> Buffer {
    device.create_buffer_init(&BufferInitDescriptor {
        label: label!("{name}VertexBuffer"),
        contents: bytemuck::cast_slice(vertices),
        usage: BufferUsages::VERTEX,
    })
}

pub fn init_indices(name: &str, indices: &[u32], device: &Device) -> Buffer {
    device.create_buffer_init(&BufferInitDescriptor {
        label: label!("{name}IndexBuffer"),
        contents: bytemuck::cast_slice(indices),
        usage: BufferUsages::INDEX,
    })
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
