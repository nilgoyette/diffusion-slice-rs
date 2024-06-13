use wgpu::CommandEncoder;

use super::*;

mod image_encoder;
mod transfer;

impl Context {
    pub fn execute_workloads(&self) {
        let mut encoder = self.encoder();

        // self.send_source_image()
        // self.render()
        self.copy_dst_texture_to_buffer(&mut encoder);

        self.client.command_queue.submit([encoder.finish()]);

        self.save_image(self.receive_image_bytes());
    }

    fn encoder(&self) -> wgpu::CommandEncoder {
        self.client
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: label!("CommandEncoder"),
            })
    }
}
