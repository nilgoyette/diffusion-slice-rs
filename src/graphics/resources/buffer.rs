use glam::vec2;
use wgpu::{Buffer, BufferUsages, Device};

use super::{vertex::ImageVertex, Texture};

pub fn create_image_vertex_buffer(device: &Device) -> Buffer {
    use wgpu::util::{BufferInitDescriptor, DeviceExt};

    device.create_buffer_init(&BufferInitDescriptor {
        label: label!("ImageVertexBuffer"),
        contents: bytemuck::cast_slice(&quad_vertices()),
        usage: BufferUsages::VERTEX,
    })
}

fn quad_vertices() -> [ImageVertex; 6] {
    let vertex = |x, y, u, v| ImageVertex {
        canon: vec2(x, y),
        uv: vec2(u, v),
    };
    [
        vertex(1., 1., 1., 1.),
        vertex(1., -1., 1., 0.),
        vertex(-1., -1., 0., 0.),
        vertex(1., 1., 1., 1.),
        vertex(-1., -1., 1., 0.),
        vertex(-1., 1., 0., 1.),
    ]
}

pub fn create_transfer_buffer(texture: &Texture, device: &Device) -> Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: label!("TransferBuffer"),
        size: texture.bytes_stride as u64 * texture.size.height as u64,
        usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
        mapped_at_creation: false,
    })
}
