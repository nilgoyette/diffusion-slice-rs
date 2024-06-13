use wgpu::Buffer;

use super::*;

pub struct ImageTransferBuffer {
    pub inner: Buffer,
    pub bytes_per_row: u32,
    pub row_count: u32,
}

impl ImageTransferBuffer {
    pub fn new(texture: &Texture, client: &Client) -> Self {
        let (bytes_per_row, row_count) = texture_bytes_size(&texture);

        let buffer = client.device.create_buffer(&wgpu::BufferDescriptor {
            label: label!("StagingBuffer"),
            size: bytes_per_row as u64 * row_count as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        ImageTransferBuffer {
            inner: buffer,
            bytes_per_row,
            row_count,
        }
    }
}

fn texture_bytes_size(texture: &Texture) -> (u32, u32) {
    // Always returns `Some(u32)` when using `Rgba8Unorm`
    let block_size = texture.format.block_copy_size(None).unwrap();

    // `bytes_per_row` must be padded to a multiple of 256
    (
        padded_bytes_size(block_size * texture.size.width, 256),
        texture.size.height,
    )
}

fn padded_bytes_size(size: u32, alignment: u32) -> u32 {
    ((size + alignment - 1) / alignment) * alignment
}
