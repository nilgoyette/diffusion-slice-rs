use glam::{UVec2, UVec3};
use wgpu::{Adapter, Device, Features, Queue};

use super::ContextInputs;
use crate::graphics::resources::COLOR_FORMAT;

/// Stores handlers related to the user environment and various parameters.
pub struct Client {
    pub device: Device,
    pub command_queue: Queue,
    pub img_size: UVec2,
    pub size_3d: UVec3,
    pub multisample_count: u32,
    pub streamline_batch_size: usize,
    pub white_mode: bool,
}

impl Client {
    pub async fn new(inputs: &ContextInputs) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let adapter = adapter(&instance).await;

        let (device, command_queue) = device(&adapter).await;

        Self {
            device,
            command_queue,
            img_size: inputs.dst_img_size,
            size_3d: inputs.size_3d,
            multisample_count: max_multisample_count(&adapter),
            streamline_batch_size: inputs.streamline_batch_size,
            white_mode: inputs.white_mode,
        }
    }
}

fn max_multisample_count(adapter: &Adapter) -> u32 {
    adapter
        .get_texture_format_features(COLOR_FORMAT)
        .flags
        .supported_sample_counts()
        .into_iter()
        .max()
        .expect("4x is always supported")
}

async fn device(adapter: &Adapter) -> (Device, Queue) {
    let required_features = Features::POLYGON_MODE_LINE
        | Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES
        | Features::ADDRESS_MODE_CLAMP_TO_BORDER;

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
