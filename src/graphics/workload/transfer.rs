use wgpu::CommandEncoder;

use crate::graphics::Context;

impl Context {
    pub(super) fn copy_target_to_buffer(&self, command_encoder: &mut CommandEncoder) {
        let texture = &self.res.target_texture;

        let copy_buffer = wgpu::ImageCopyBuffer {
            buffer: &self.res.transfer_buffer,
            layout: texture.data_layout(),
        };
        command_encoder.copy_texture_to_buffer(
            texture.image_copy(),
            copy_buffer,
            self.res.target_texture.inner.size(),
        );
    }

    pub(super) fn receive_image_bytes(&self) -> Vec<u8> {
        let buffer = &self.res.transfer_buffer;
        let data = buffer.slice(..);

        data.map_async(wgpu::MapMode::Read, |result| {
            result.expect("The buffer can be mapped")
        });
        self.client.device.poll(wgpu::Maintain::Wait); // Synchronization

        let texture = &self.res.target_texture;
        let bytes_width = (texture.bytes_stride - texture.bytes_padding) as usize;

        let bytes = data
            .get_mapped_range()
            .chunks(texture.bytes_stride as usize)
            .flat_map(|chunk| chunk[..bytes_width].iter().copied())
            .collect();

        buffer.unmap();

        bytes
    }
}
