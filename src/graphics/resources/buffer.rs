use glam::{vec2, UVec2, Vec2};
use wgpu::{Buffer, BufferUsages, Device};

use super::{vertex::ImageVertex, Client, Texture};

pub fn create_image_vertex_buffer(size: UVec2, client: &Client) -> Buffer {
    use wgpu::util::{BufferInitDescriptor, DeviceExt};

    let uv_offset = uv_offset(size, client.img_size);

    client.device.create_buffer_init(&BufferInitDescriptor {
        label: label!("ImageVertexBuffer"),
        contents: bytemuck::cast_slice(&quad_vertices(uv_offset)),
        usage: BufferUsages::VERTEX,
    })
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
    [
        vertex(1., 1., 0. - du, 1. + dv),
        vertex(1., -1., 0. - du, 0. - dv),
        vertex(-1., -1., 1. + du, 0. - dv),
        vertex(1., 1., 0. - du, 1. + dv),
        vertex(-1., -1., 1. + du, 0. - dv),
        vertex(-1., 1., 1. + du, 1. + dv),
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
