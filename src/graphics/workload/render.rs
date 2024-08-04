use wgpu::{CommandEncoder, Operations};

use crate::{
    graphics::{
        resources::{bind, Resources, Texture},
        Context,
    },
    ImageSlice,
};

impl Context {
    pub(super) fn render_slice(&self, image: &ImageSlice, command_encoder: &mut CommandEncoder) {
        let source_bind_group = bind::group::source(image, &self);
        let transform_bind_group = bind::group::transform(&self);
        {
            let mut pass = render_pass(&self.res, command_encoder);

            // Resampling
            pass.set_bind_group(0, &source_bind_group, &[]);

            pass.set_pipeline(&self.pipelines.resampling);
            pass.set_vertex_buffer(0, self.res.image_vertices.slice(..));
            pass.draw(0..6, 0..1);

            // Streamline
            if let Some(fibers) = &self.res.fibers {
                pass.set_bind_group(0, &transform_bind_group, &[]);

                pass.set_pipeline(&self.pipelines.streamline);
                pass.set_vertex_buffer(0, fibers.vertices.slice(..));
                pass.set_index_buffer(fibers.indices.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..fibers.index_count, 0, 0..1);
            }
        }
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
        depth_stencil_attachment: Some(depth_attachment(&res.depth_texture)),
        ..Default::default()
    })
}

fn depth_attachment(texture: &Texture) -> wgpu::RenderPassDepthStencilAttachment {
    wgpu::RenderPassDepthStencilAttachment {
        view: &texture.view,
        depth_ops: Some(Operations {
            load: wgpu::LoadOp::Clear(1.),
            store: wgpu::StoreOp::Discard,
        }),
        stencil_ops: None,
    }
}

fn clear<T>(value: T) -> Operations<T> {
    Operations {
        load: wgpu::LoadOp::Clear(value),
        store: wgpu::StoreOp::Store,
    }
}
