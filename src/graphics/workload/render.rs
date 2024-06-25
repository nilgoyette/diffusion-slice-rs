use wgpu::Operations;

use super::*;

impl Context {
    pub(super) fn render(&self, command_encoder: &mut CommandEncoder) {
        let mut pass = render_pass(&self.res, command_encoder);

        pass.set_pipeline(&self.pipelines.resampling);
        pass.set_vertex_buffer(0, self.res.image_vertex_buffer.slice(..));
        pass.draw(0..6, 0..1);

        // TODO self.render_lines()
        // TODO self.post_process()
    }
}

fn render_pass<'a>(
    res: &'a Resources,
    command_encoder: &'a mut wgpu::CommandEncoder,
) -> wgpu::RenderPass<'a> {
    let color_attachment = wgpu::RenderPassColorAttachment {
        view: &res.multisampled_texture.view,
        resolve_target: Some(&res.target_texture.view),
        ops: clear(wgpu::Color::BLACK),
    };
    command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: label!("RenderPass"),
        color_attachments: &[Some(color_attachment)],
        depth_stencil_attachment: None,
        ..Default::default()
    })
}

fn clear<T>(value: T) -> Operations<T> {
    Operations {
        load: wgpu::LoadOp::Clear(value),
        store: wgpu::StoreOp::Store,
    }
}
