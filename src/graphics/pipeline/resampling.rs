use super::*;

pub fn state<'a>(color_format: TextureFormat) -> PipelineState<'a, ImageVertex> {
    PipelineState {
        name: "Resampling",
        shader_code: include_str!("shaders/resampling.wgsl"),
        color_target: color_target(color_format),
        depth_stencil: None,
        primitive: triangle_primitive(),
        alpha_to_coverage: true,
        _vertex_type: PhantomData,
    }
}

fn color_target(format: TextureFormat) -> ColorTargetState {
    ColorTargetState {
        format,
        blend: Some(BlendState::REPLACE),
        write_mask: ColorWrites::ALL,
    }
}
