use wgpu::CommandEncoder;

use super::{Context, Image};

mod render;
mod transfer;

impl Context {
    pub fn execute_workloads(&self) -> Image {
        let mut command_encoder = self.command_encoder();

        self.render(&mut command_encoder);
        self.copy_target_to_buffer(&mut command_encoder);

        self.client.command_queue.submit([command_encoder.finish()]);

        let size = self.client.img_size;

        Image::from_raw(size.x, size.y, self.receive_image_bytes())
            .expect("Data size must match image dimensions")
    }

    fn command_encoder(&self) -> CommandEncoder {
        self.client
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: label!("CommandEncoder"),
            })
    }
}
