use glam::vec2;
use wgpu::{Buffer, BufferUsages, Device};

use super::{vertex::ImageVertex, Client, Texture};

pub fn create_image_vertex_buffer(size: (u32, u32), client: &Client) -> Buffer {
    use wgpu::util::{BufferInitDescriptor, DeviceExt};

    client.device.create_buffer_init(&BufferInitDescriptor {
        label: label!("ImageVertexBuffer"),
        contents: bytemuck::cast_slice(&quad_vertices(size, client.img_size)),
        usage: BufferUsages::VERTEX,
    })
}

fn quad_vertices(src_size: (u32, u32), dst_size: (u32, u32)) -> [ImageVertex; 6] {
    let (src_w, src_h, dst_w, dst_h) = {
        let ((src_w, src_h), (dst_w, dst_h)) = (src_size, dst_size);
        (src_w as f32, src_h as f32, dst_w as f32, dst_h as f32)
    };
    let (a, b) = (dst_w / src_w, dst_h / src_h);

    let (du, dv) = if a > b {
        let c = src_w * b;
        ((dst_w - c) / (2. * c), 0.)
    } else {
        let c = src_h * a;
        (0., (dst_h - c) / (2. * c))
    };
    let vertex = |x, y, uv| ImageVertex {
        canon: vec2(x, y),
        uv,
    };
    [
        vertex(1., 1., vec2(1. + du, 0. - dv)),
        vertex(1., -1., vec2(1. + du, 1. + dv)),
        vertex(-1., -1., vec2(0. - du, 1. + dv)),
        vertex(1., 1., vec2(1. + du, 0. - dv)),
        vertex(-1., -1., vec2(0. - du, 1. + dv)),
        vertex(-1., 1., vec2(0. - du, 0. - dv)),
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
