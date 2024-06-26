use std::marker::PhantomData;

use wgpu::{BlendState, ColorTargetState, ColorWrites, TextureFormat};

use super::{triangle_primitive, Binding, PipelineState};
use crate::graphics::resources::vertex::ImageVertex;

pub fn state(color_format: TextureFormat, binding: &Binding) -> PipelineState<ImageVertex> {
    PipelineState {
        name: "Resampling",
        shader_code: include_str!("shaders/resampling.wgsl"),
        bindings: vec![binding],
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
