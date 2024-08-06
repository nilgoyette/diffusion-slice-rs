use std::marker::PhantomData;

use wgpu::{FrontFace, PolygonMode, PrimitiveState, PrimitiveTopology};

use crate::graphics::resources::vertex::{FiberVertex, ImageVertex, Vertex};

pub struct PipelineState<'a, V: Vertex> {
    pub name: &'a str,

    /// WGSL code. Generally included with `include_str!(...)`
    pub shader_code: &'a str,

    /// Specifies the bind groups to the pipeline layout
    pub bindings: Vec<&'a str>,

    pub use_depth_test: bool,
    pub primitive: PrimitiveState,

    pub _vertex_type: PhantomData<V>,
}

fn triangle_primitive() -> PrimitiveState {
    PrimitiveState {
        topology: PrimitiveTopology::TriangleList,
        front_face: FrontFace::Cw,
        polygon_mode: PolygonMode::Fill,
        ..Default::default()
    }
}

fn line_primitive() -> PrimitiveState {
    PrimitiveState {
        topology: PrimitiveTopology::LineList,
        polygon_mode: PolygonMode::Line,
        ..Default::default()
    }
}

pub fn resampling<'a>() -> PipelineState<'a, ImageVertex> {
    PipelineState {
        name: "Resampling",
        shader_code: include_str!("shaders/resampling.wgsl"),
        bindings: vec!["Source"],
        primitive: triangle_primitive(),
        use_depth_test: false,
        _vertex_type: PhantomData,
    }
}

pub fn streamline<'a>() -> PipelineState<'a, FiberVertex> {
    PipelineState {
        name: "Streamline",
        shader_code: include_str!("shaders/streamline.wgsl"),
        bindings: vec!["Transform"],
        primitive: line_primitive(),
        use_depth_test: true,
        _vertex_type: PhantomData,
    }
}
