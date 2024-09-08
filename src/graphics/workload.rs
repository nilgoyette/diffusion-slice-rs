use bytemuck::Pod;
use glam::Mat3;
use wgpu::{Buffer, CommandEncoder};

use crate::graphics::{resources::quad_vertices, Context, Image, Slice};

mod render;
mod transfer;

impl Context {
    pub fn execute_workloads(&self, slice: &Slice) -> Image {
        let mut command_encoder = self.command_encoder();

        self.update_slice_data(slice);
        self.render_slice(&slice.data, &mut command_encoder);
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

    pub fn update_slice_data(&self, slice: &Slice) {
        let vertices = quad_vertices(self.slice_transform(slice));

        let transform = self.parameters.tractogram_projection
            * slice.view.rotation() // Rotates the tractogram according to the view
            * self.parameters.tractogram_alignment;

        self.write(&self.res.image_vertices, vertices);
        self.write(&self.res.transform, transform);
    }

    fn slice_transform(&self, slice: &Slice) -> Mat3 {
        let screen_space_scale =
            self.parameters.fit_scale * slice.size().as_vec2() / self.client.img_size.as_vec2();

        slice.view.orientation() * Mat3::from_scale(screen_space_scale)
    }

    fn write<T: Pod>(&self, buffer: &Buffer, data: T) {
        let data = &[data];
        let bytes = bytemuck::cast_slice(data);

        self.client.command_queue.write_buffer(buffer, 0, bytes);
    }
}
