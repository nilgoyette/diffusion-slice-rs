use wgpu::{Adapter, Device, Queue};

use super::UserInputs;
use crate::graphics::resources::COLOR_FORMAT;

/// Stores handlers related to the user environment
pub struct Client {
    pub device: Device,
    pub command_queue: Queue,
    pub img_size: (u32, u32),
    pub multisample_count: u32,
}

impl Client {
    pub async fn new(inputs: &UserInputs) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let adapter = adapter(&instance).await;

        let (device, command_queue) = device(&adapter).await;

        Self {
            device,
            command_queue,
            img_size: inputs.dst_img_size,
            multisample_count: max_supported_multisample_count(&adapter),
        }
    }
}

fn max_supported_multisample_count(adapter: &Adapter) -> u32 {
    adapter
        .get_texture_format_features(COLOR_FORMAT)
        .flags
        .supported_sample_counts()
        .into_iter()
        .max()
        .expect("At least one multisample count should be supported")
}

async fn device(adapter: &Adapter) -> (Device, Queue) {
    use wgpu::Features;

    let required_features =
        Features::POLYGON_MODE_LINE | Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES;

    let desc = &wgpu::DeviceDescriptor {
        label: None,
        required_features,
        required_limits: wgpu::Limits::default(),
    };
    // Tracing is disabled
    adapter.request_device(desc, None).await.unwrap()
}

async fn adapter(instance: &wgpu::Instance) -> Adapter {
    let options = &wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
    };
    instance.request_adapter(options).await.unwrap()
}
