use std::marker::PhantomData;

use wgpu::{BlendState, ColorTargetState, ColorWrites, TextureFormat};

use super::{triangle_primitive, PipelineState};
use crate::graphics::resources::vertex::ImageVertex;

pub fn state<'a>(color_format: TextureFormat) -> PipelineState<'a, ImageVertex> {
    PipelineState {
        name: "Resampling",
        shader_code: include_str!("shaders/resampling.wgsl"),
        bind_layouts: vec!["Source"],
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
