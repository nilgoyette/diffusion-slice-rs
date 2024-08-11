use wgpu::{
    AddressMode, BindGroup, BindGroupEntry, BindingResource, FilterMode, SamplerBorderColor,
};

use crate::graphics::{resources::Texture, Client, Context, ImageSlice};

pub fn source(image: &ImageSlice, ctx: &Context) -> BindGroup {
    let sampler = create_sampler(&ctx.client);
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

fn create_sampler(client: &Client) -> wgpu::Sampler {
    let border_color = if client.white_mode {
        SamplerBorderColor::OpaqueWhite
    } else {
        SamplerBorderColor::OpaqueBlack
    };
    client.device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: AddressMode::ClampToBorder,
        address_mode_v: AddressMode::ClampToBorder,
        mag_filter: FilterMode::Linear,
        min_filter: FilterMode::Linear,
        border_color: Some(border_color),
        ..Default::default()
    })
}

fn create_bind_group(key: &str, entries: Vec<BindingResource>, ctx: &Context) -> BindGroup {
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
