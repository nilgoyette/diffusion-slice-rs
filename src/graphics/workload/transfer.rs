use super::*;

impl Context {
    pub(super) fn copy_target_to_buffer(&self, command_encoder: &mut CommandEncoder) {
        let transfer_buffer = &self.res.transfer_buffer;

        let copy_texture = wgpu::ImageCopyTexture {
            texture: &self.res.target_texture.inner,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        };
        let copy_buffer = wgpu::ImageCopyBuffer {
            buffer: &transfer_buffer.inner,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(transfer_buffer.bytes_per_row),
                rows_per_image: Some(transfer_buffer.row_count),
            },
        };
        command_encoder.copy_texture_to_buffer(
            copy_texture,
            copy_buffer,
            self.res.target_texture.size,
        );
    }

    pub(super) fn receive_image_bytes(&self) -> Vec<u8> {
        let buffer = &self.res.transfer_buffer.inner;
        let data = buffer.slice(..);

        // TODO handle the error in the main thread
        data.map_async(wgpu::MapMode::Read, |result| result.unwrap());

        self.client.device.poll(wgpu::Maintain::Wait); // Synchronization

        let bytes: Vec<u8> = data.get_mapped_range().to_vec();
        buffer.unmap();

        bytes
    }
}
