use wgpu::{BindGroup, BindGroupLayout, BindingResource, BindingType, Device, ShaderStages};

use super::Texture;

struct Entry<'a> {
    stage: ShaderStages,
    binding_type: BindingType,
    resource: BindingResource<'a>,
}

pub struct Binding {
    pub group: BindGroup,
    pub layout: BindGroupLayout,
}

impl Binding {
    pub fn new(source_texture: &Texture, device: &Device) -> Self {
        let sampler = create_sampler(device);

        let entries = vec![
            Entry {
                stage: ShaderStages::FRAGMENT,
                binding_type: BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                resource: BindingResource::TextureView(&source_texture.view),
            },
            Entry {
                stage: ShaderStages::FRAGMENT,
                binding_type: BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                resource: BindingResource::Sampler(&sampler),
            },
        ];
        let layout = create_layout(&entries, device);

        Self {
            group: create_bind_group(entries, &layout, device),
            layout,
        }
    }
}

fn create_bind_group(entries: Vec<Entry>, layout: &BindGroupLayout, device: &Device) -> BindGroup {
    use wgpu::BindGroupEntry;

    let entries: Vec<BindGroupEntry> = entries
        .into_iter()
        .enumerate()
        .map(|(i, entry)| BindGroupEntry {
            binding: i as u32,
            resource: entry.resource,
        })
        .collect();

    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: label!("BindingGroup"),
        layout,
        entries: &entries,
    })
}

fn create_layout(entries: &[Entry], device: &Device) -> BindGroupLayout {
    use wgpu::{BindGroupLayoutDescriptor, BindGroupLayoutEntry};

    let entry = |(i, entry): (usize, &Entry)| BindGroupLayoutEntry {
        binding: i as u32,
        visibility: entry.stage,
        ty: entry.binding_type,
        count: None, // For arrays
    };
    let entries: Vec<BindGroupLayoutEntry> = entries.iter().enumerate().map(entry).collect();

    device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: label!("BindingLayout"),
        entries: &entries,
    })
}

fn create_sampler(device: &Device) -> wgpu::Sampler {
    use wgpu::{AddressMode, FilterMode};

    device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: AddressMode::ClampToEdge,
        address_mode_v: AddressMode::ClampToEdge,
        mag_filter: FilterMode::Linear,
        min_filter: FilterMode::Linear,
        ..Default::default()
    })
}
