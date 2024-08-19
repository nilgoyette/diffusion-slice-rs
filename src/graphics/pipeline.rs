use wgpu::{
    BindGroupLayout, ColorTargetState, CompareFunction, DepthStencilState, Device,
    MultisampleState, RenderPipeline,
};

use super::{
    resources::{vertex::Vertex, Resources, COLOR_FORMAT, DEPTH_FORMAT},
    Client,
};

use state::PipelineState;

mod state;

pub struct Pipelines {
    pub resampling: RenderPipeline,
    pub streamline: Option<RenderPipeline>,
    // pub post_processing: RenderPipeline,
}

impl Pipelines {
    pub fn new(res: &Resources, client: &Client) -> Self {
        let streamline =
            (!res.fibers.is_empty()).then(|| create_pipeline(state::streamline(), res, client));

        Self {
            resampling: create_pipeline(state::resampling(), res, client),
            streamline,
        }
    }
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
        targets: &[Some(color_target())],
        compilation_options: Default::default(),
    };
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: label!("{}Pipeline", state.name),
        layout: Some(&layout(state.name, state.bindings, res, device)),
        vertex: vertex_state,
        fragment: Some(fragment_state),
        primitive: state.primitive,
        depth_stencil: Some(depth_stencil(state.use_depth_test)),
        multisample: multisample(client.multisample_count),
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

fn color_target() -> ColorTargetState {
    ColorTargetState {
        format: COLOR_FORMAT,
        blend: Some(wgpu::BlendState::REPLACE),
        write_mask: wgpu::ColorWrites::ALL,
    }
}

fn depth_stencil(active: bool) -> DepthStencilState {
    let depth_compare = if active {
        CompareFunction::LessEqual
    } else {
        CompareFunction::Always
    };
    DepthStencilState {
        format: DEPTH_FORMAT,
        depth_write_enabled: active,
        depth_compare,
        stencil: wgpu::StencilState::default(),
        bias: wgpu::DepthBiasState::default(),
    }
}

fn multisample(count: u32) -> MultisampleState {
    MultisampleState {
        count,
        mask: !0,
        alpha_to_coverage_enabled: false,
    }
}
