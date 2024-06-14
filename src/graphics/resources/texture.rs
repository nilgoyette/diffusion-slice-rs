use wgpu::{Extent3d, TextureFormat, TextureUsages};

use super::*;

struct TextureConfig {
    name: String,
    usage: TextureUsages,
    format: TextureFormat,
    size: Extent3d,
    multisampled: bool,
}

pub struct Texture {
    pub inner: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub format: TextureFormat,
    pub size: Extent3d,
}

impl Texture {
    pub fn new_target(client: &Client) -> Self {
        let size = extent(client.img_size);
        let format = TextureFormat::Rgba8Unorm;

        let texture = client.create_texture(TextureConfig {
            name: "Destination".to_string(),
            usage: TextureUsages::COPY_SRC | TextureUsages::RENDER_ATTACHMENT,
            format,
            size,
            multisampled: false,
        });

        Self {
            view: view(&texture),
            inner: texture,
            format,
            size,
        }
    }
}

impl Client {
    fn create_texture(&self, cfg: TextureConfig) -> wgpu::Texture {
        let sample_count = match cfg.multisampled {
            true => MULTISAMPLE_COUNT,
            false => 1,
        };
        self.device.create_texture(&wgpu::TextureDescriptor {
            label: label!("{:?}Texture", cfg.name),
            size: cfg.size,
            mip_level_count: 1,
            sample_count,
            dimension: wgpu::TextureDimension::D2,
            format: cfg.format,
            usage: cfg.usage,
            view_formats: &[],
        })
    }
}

fn extent(size: (u32, u32)) -> Extent3d {
    let (width, height) = size;
    wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    }
}

fn view(texture: &wgpu::Texture) -> wgpu::TextureView {
    texture.create_view(&wgpu::TextureViewDescriptor::default())
}
