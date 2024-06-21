use super::*;

impl Context {
    pub(super) fn copy_target_to_buffer(&self, command_encoder: &mut CommandEncoder) {
        let texture = &self.res.target_texture;

        let copy_buffer = wgpu::ImageCopyBuffer {
            buffer: &self.res.transfer_buffer,
            layout: texture.data_layout(true),
        };
        command_encoder.copy_texture_to_buffer(
            texture.image_copy(),
            copy_buffer,
            self.res.target_texture.size,
        );
    }

    pub(super) fn receive_image_bytes(&self) -> Vec<u8> {
        let buffer = &self.res.transfer_buffer;
        let data = buffer.slice(..);

        data.map_async(wgpu::MapMode::Read, |result| {
            result.expect("Failed to map buffer")
        });

        self.client.device.poll(wgpu::Maintain::Wait); // Synchronization

        let bytes: Vec<u8> = data.get_mapped_range().to_vec();
        buffer.unmap();

        bytes
    }
}
