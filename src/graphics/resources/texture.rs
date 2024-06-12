pub use wgpu::TextureFormat;
use wgpu::TextureUsages;

use super::*;

pub const DEPTH_STENCIL_FORMAT: TextureFormat = TextureFormat::Depth24PlusStencil8;

pub struct TextureConfig {
    pub name: String,
    pub usage: wgpu::TextureUsages,
    pub format: wgpu::TextureFormat,
    pub size: (u32, u32),
    pub multisampled: bool,
}

pub struct TextureData {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
}

impl TextureData {
    pub fn new_dst(client: &Client) -> Self {
        let texture = client.texture(TextureConfig {
            name: "Destination".to_string(),
            usage: TextureUsages::COPY_SRC | TextureUsages::RENDER_ATTACHMENT,
            format: TextureFormat::Rgba32Float,
            size: client.dst_img_size,
            multisampled: false,
        });
        Self {
            view: view(&texture),
            texture,
        }
    }
}

impl Client {
    fn texture(&self, cfg: TextureConfig) -> wgpu::Texture {
        let size = {
            let (width, height) = cfg.size;
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            }
        };
        let sample_count = match cfg.multisampled {
            true => MULTISAMPLE_COUNT,
            false => 1,
        };
        self.device.create_texture(&wgpu::TextureDescriptor {
            label: label!("{:?}Texture", cfg.name),
            size,
            mip_level_count: 1,
            sample_count,
            dimension: wgpu::TextureDimension::D2,
            format: cfg.format,
            usage: cfg.usage,
            view_formats: &[],
        })
    }
}

fn view(texture: &wgpu::Texture) -> wgpu::TextureView {
    texture.create_view(&wgpu::TextureViewDescriptor::default())
}
