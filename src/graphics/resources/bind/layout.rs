use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Device,
    ShaderStages,
};

struct LayoutEntry {
    stage: ShaderStages,
    binding_type: BindingType,
}

pub fn source(device: &Device) -> BindGroupLayout {
    let entries = vec![
        LayoutEntry {
            stage: ShaderStages::FRAGMENT,
            binding_type: BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
        },
        LayoutEntry {
            stage: ShaderStages::FRAGMENT,
            binding_type: BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
        },
    ];
    create_layout("Source", &entries, device)
}

pub fn transform(device: &Device) -> BindGroupLayout {
    let entries = vec![LayoutEntry {
        stage: ShaderStages::VERTEX,
        binding_type: BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
    }];
    create_layout("Transform", &entries, device)
}

fn create_layout(name: &str, entries: &[LayoutEntry], device: &Device) -> BindGroupLayout {
    let entry = |(i, entry): (usize, &LayoutEntry)| BindGroupLayoutEntry {
        binding: i as u32,
        visibility: entry.stage,
        ty: entry.binding_type,
        count: None, // For arrays
    };
    let entries: Vec<BindGroupLayoutEntry> = entries.iter().enumerate().map(entry).collect();

    device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: label!("{name}BindingLayout"),
        entries: &entries,
    })
}
