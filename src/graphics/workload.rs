use bytemuck::Pod;
use glam::{vec2, Mat3, Mat4, Vec2, Vec3};
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
        let client = &self.client;

        let (src_size, dst_size) = (slice.size().as_vec2(), client.img_size.as_vec2());
        let size_3d = client.size_3d.as_vec3();

        let scale = scale(dst_size, size_3d);

        let vertices = {
            let quad_transform = slice.view.orientation()
                * screen_space_transform(dst_size)
                * Mat3::from_translation((dst_size - scale * src_size) / 2.)
                * Mat3::from_scale(vec2(scale, scale));

            quad_vertices(src_size, quad_transform)
        };
        let transform = fiber_projection(dst_size, size_3d, scale)
            * Mat4::from_scale(Vec3::splat(scale))
            * slice.view.rotation()
            * Mat4::from_translation(-size_3d / 2.);

        self.write(&self.res.image_vertices, vertices);
        self.write(&self.res.transform, transform);
    }

    fn write<T: Pod>(&self, buffer: &Buffer, data: T) {
        let data = &[data];
        let bytes = bytemuck::cast_slice(data);

        self.client.command_queue.write_buffer(buffer, 0, bytes);
    }
}

/// Calculates the scaling factor with respect to
/// the aspect ratio and uniformity across the three axes.
fn scale(dst_size: Vec2, size_3d: Vec3) -> f32 {
    let max_size = vec2(size_3d.x, size_3d.z).max(vec2(size_3d.y, size_3d.y));
    (dst_size / max_size).min_element()
}

/// Find the change-of-basis matrix for the specified destination size.
fn screen_space_transform(dst_size: Vec2) -> Mat3 {
    Mat3::from_translation(vec2(-1., -1.)) * Mat3::from_scale(2. / dst_size)
}

/// Creates an orthographic projection matrix aligned with the slice images.
fn fiber_projection(dst_size: Vec2, size_3d: Vec3, scale: f32) -> Mat4 {
    let rect = dst_size / 2.;
    let depth = scale * size_3d.max_element() / 2.;
    Mat4::orthographic_rh(-rect.x, rect.x, -rect.y, rect.y, -depth, depth)
}
