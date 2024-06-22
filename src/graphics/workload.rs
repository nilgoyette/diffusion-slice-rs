use wgpu::CommandEncoder;

use super::Context;

mod save;
mod transfer;

impl Context {
    pub fn execute_workloads(&self) {
        let mut command_encoder = self.command_encoder();

        // TODO self.resample_source_image()
        // TODO self.render_lines()
        // TODO self.post_process()
        self.copy_target_to_buffer(&mut command_encoder);

        self.client.command_queue.submit([command_encoder.finish()]);

        self.save_image(self.receive_image_bytes());
    }

    fn command_encoder(&self) -> wgpu::CommandEncoder {
        self.client
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: label!("CommandEncoder"),
            })
    }
}
