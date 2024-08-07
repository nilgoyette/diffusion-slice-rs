use std::marker::PhantomData;

use wgpu::{
    BindGroupLayout, ColorTargetState, DepthStencilState, Device, PrimitiveState, RenderPipeline,
};

use super::{
    resources::{vertex::Vertex, Resources},
    Client,
};

mod resampling;

pub struct Pipelines {
    pub resampling: RenderPipeline,
    // pub lines: RenderPipeline,
    // pub post_processing: RenderPipeline,
}

impl Pipelines {
    pub fn new(res: &Resources, client: &Client) -> Self {
        let fmt = res.target_texture.format;

        Self {
            resampling: create_pipeline(resampling::state(fmt), res, client),
        }
    }
}

pub struct PipelineState<'a, V: Vertex> {
    pub name: &'a str,

    /// WGSL code. Generally included with `include_str!(...)`
    pub shader_code: &'a str,

    pub bind_layouts: Vec<&'a str>,

    pub color_target: ColorTargetState,
    pub depth_stencil: Option<DepthStencilState>,
    pub primitive: PrimitiveState,

    pub alpha_to_coverage: bool,

    pub _vertex_type: PhantomData<V>,
}

fn create_pipeline<V: Vertex>(
    state: PipelineState<V>,
    res: &Resources,
    client: &Client,
) -> RenderPipeline {
    let device = &client.device;
    let module = &shader_module(state.name, state.shader_code, device);
    let vertex_attributes = V::attributes();

    let vertex_state = wgpu::VertexState {
        module,
        entry_point: "vertex",
        buffers: &[V::buffer_layout(&vertex_attributes)],
        compilation_options: Default::default(),
    };
    let fragment_state = wgpu::FragmentState {
        module,
        entry_point: "fragment",
        targets: &[Some(state.color_target)],
        compilation_options: Default::default(),
    };
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: label!("{}Pipeline", state.name),
        layout: Some(&layout(state.name, state.bind_layouts, res, device)),
        vertex: vertex_state,
        fragment: Some(fragment_state),
        primitive: state.primitive,
        depth_stencil: state.depth_stencil,
        multisample: multisample(client.multisample_count, state.alpha_to_coverage),
        multiview: None,
    })
}

fn shader_module(name: &str, code: &str, device: &Device) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: label!("{}", name),
        source: wgpu::ShaderSource::Wgsl(code.into()),
    })
}

fn layout(
    name: &str,
    bind_layouts: Vec<&str>,
    res: &Resources,
    device: &Device,
) -> wgpu::PipelineLayout {
    let bind_group_layouts: &Vec<&BindGroupLayout> = &bind_layouts
        .into_iter()
        .map(|id| &res.bind_layouts[id])
        .collect();

    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: label!("{}PipelineLayout", name),
        bind_group_layouts,
        push_constant_ranges: &[],
    })
}

fn multisample(count: u32, alpha_to_coverage_enabled: bool) -> wgpu::MultisampleState {
    wgpu::MultisampleState {
        count,
        mask: !0,
        alpha_to_coverage_enabled,
    }
}

fn triangle_primitive() -> PrimitiveState {
    PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList,
        front_face: wgpu::FrontFace::Ccw,
        polygon_mode: wgpu::PolygonMode::Fill,
        cull_mode: Some(wgpu::Face::Front),
        ..Default::default()
    }
}
