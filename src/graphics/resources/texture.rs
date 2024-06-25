use wgpu::{Extent3d, ImageCopyTexture, ImageDataLayout, Queue, TextureFormat, TextureUsages};

use crate::graphics::{Client, Image, MULTISAMPLE_COUNT};

struct TextureConfig {
    name: String,
    usage: TextureUsages,
    format: TextureFormat,
    size: Extent3d,
    multisampled: bool,

    /// Required for `copy_texture_to_buffer`, the size must be aligned to 256.
    pad_bytes_per_row: bool,
}

pub struct Texture {
    pub inner: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub format: TextureFormat,
    pub size: Extent3d,

    pub bytes_stride: u32,
    pub bytes_padding: u32,
}

impl Texture {
    fn new(cfg: TextureConfig, client: &Client) -> Self {
        let texture = client.create_texture(&cfg);
        let (bytes_stride, bytes_padding) = bytes_layout(&cfg);

        Self {
            view: view(&texture),
            inner: texture,
            format: cfg.format,
            size: cfg.size,

            bytes_stride,
            bytes_padding,
        }
    }

    pub fn new_source(image: &Image, client: &Client) -> Self {
        let cfg = TextureConfig {
            name: "SourceTexture".to_string(),
            usage: TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING,
            format: TextureFormat::Rgba8Unorm,
            size: extent(image.dimensions()),
            multisampled: false,
            pad_bytes_per_row: false,
        };
        let texture = Self::new(cfg, client);
        texture.send_image(image, &client.command_queue);

        texture
    }

    pub fn new_multisampled(client: &Client) -> Self {
        let cfg = TextureConfig {
            name: "MultisampledTexture".to_string(),
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: TextureFormat::Rgba8Unorm,
            size: extent(client.img_size),
            multisampled: true,
            pad_bytes_per_row: false,
        };
        Self::new(cfg, client)
    }

    pub fn new_target(client: &Client) -> Self {
        let cfg = TextureConfig {
            name: "TargetTexture".to_string(),
            usage: TextureUsages::COPY_SRC | TextureUsages::RENDER_ATTACHMENT,
            format: TextureFormat::Rgba8Unorm,
            size: extent(client.img_size),
            multisampled: false,
            pad_bytes_per_row: true,
        };
        Self::new(cfg, client)
    }

    fn send_image(&self, image: &Image, command_queue: &Queue) {
        command_queue.write_texture(self.image_copy(), image, self.data_layout(), self.size);
    }

    pub fn image_copy(&self) -> ImageCopyTexture {
        ImageCopyTexture {
            texture: &self.inner,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        }
    }

    pub fn data_layout(&self) -> ImageDataLayout {
        ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(self.bytes_stride),
            rows_per_image: None,
        }
    }
}

fn view(texture: &wgpu::Texture) -> wgpu::TextureView {
    texture.create_view(&wgpu::TextureViewDescriptor::default())
}

/// Returns the number of bytes per row and the padding width
fn bytes_layout(cfg: &TextureConfig) -> (u32, u32) {
    // Always returns `Some(u32)` when using `Rgba8Unorm`
    let block_size = cfg
        .format
        .block_copy_size(None)
        .expect("Bad texture format");

    let width = block_size * cfg.size.width;

    match cfg.pad_bytes_per_row {
        true => {
            let stride = pad_size(width, wgpu::COPY_BYTES_PER_ROW_ALIGNMENT);
            (stride, stride - width)
        }
        false => (width, 0),
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

fn pad_size(size: u32, align: u32) -> u32 {
    ((size + align - 1) / align) * align
}

impl Client {
    fn create_texture(&self, cfg: &TextureConfig) -> wgpu::Texture {
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
