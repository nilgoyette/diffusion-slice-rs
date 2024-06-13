use super::*;

impl Context {
    pub(super) fn copy_texture_to_buffer<'a>(&self, encoder: &'a mut CommandEncoder) {
        let transfer = &self.res.transfer_buffer;

        let copy_texture = wgpu::ImageCopyTexture {
            texture: &self.res.dst_texture.inner,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        };
        let copy_buffer = wgpu::ImageCopyBuffer {
            buffer: &transfer.inner,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(transfer.bytes_per_row),
                rows_per_image: Some(transfer.row_count),
            },
        };
        encoder.copy_texture_to_buffer(copy_texture, copy_buffer, self.res.dst_texture.size);
    }

    pub(super) fn receive_image_bytes<'a>(&self, encoder: &'a mut CommandEncoder) -> Vec<u8> {
        let data = self.res.transfer_buffer.inner.slice(..);

        // TODO handle the error in the main thread
        data.map_async(wgpu::MapMode::Read, |result| result.unwrap());

        self.client.device.poll(wgpu::Maintain::Wait);

        let bytes: Vec<u8> = data.get_mapped_range().to_vec();

        staging_buffer.unmap();

        bytes
    }

    pub fn transfer(&self) {
        // self.client.command_queue.submit(Some(encoder.finish()));

        let img: image::RgbaImage = image::ImageBuffer::from_raw(
            self.client.img_size.width,
            self.client.img_size.height,
            bytes,
        )
        .expect("Raw data has incorrect length");

        img.save("output.png").expect("Failed to save image");

        // println!("Image saved as output.png");
    }
}
