use super::*;

pub fn create_image_transfer_buffer(texture: &Texture, device: &Device) -> Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: label!("ImageTransferBuffer"),
        size: texture.bytes_per_row as u64 * texture.size.height as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    })
}
