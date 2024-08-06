use wgpu::{BindGroup, BindingResource, Device};

use crate::graphics::{resources::Texture, Context, ImageSlice};

pub fn source(image: &ImageSlice, ctx: &Context) -> BindGroup {
    let sampler = create_sampler(&ctx.client.device);
    let source_texture = Texture::new_source(image, &ctx.client);

    let entries = vec![
        BindingResource::TextureView(&source_texture.view),
        BindingResource::Sampler(&sampler),
    ];
    create_bind_group("Source", entries, ctx)
}

pub fn transform(ctx: &Context) -> BindGroup {
    let entries = vec![ctx.res.transform.as_entire_binding()];
    create_bind_group("Transform", entries, ctx)
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

fn create_bind_group(key: &str, entries: Vec<BindingResource>, ctx: &Context) -> BindGroup {
    use wgpu::BindGroupEntry;

    let entries: Vec<BindGroupEntry> = entries
        .into_iter()
        .enumerate()
        .map(|(i, entry)| BindGroupEntry {
            binding: i as u32,
            resource: entry,
        })
        .collect();

    ctx.client
        .device
        .create_bind_group(&wgpu::BindGroupDescriptor {
            label: label!("{key}BindGroup"),
            layout: &ctx.res.bind_layouts[key],
            entries: &entries,
        })
}
