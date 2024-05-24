use super::*;

/// Stores handlers related to the user environment
pub struct Client {
    pub device: Device,
    pub command_queue: Queue,
}

impl Client {
    pub async fn new() -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

        let (device, command_queue) = {
            let adapter = adapter(&instance).await;
            device(&adapter).await
        };

        Self {
            device,
            command_queue,
        }
    }
}

async fn device(adapter: &wgpu::Adapter) -> (Device, Queue) {
    use wgpu::Features;

    let desc = &wgpu::DeviceDescriptor {
        label: None,
        required_features: Features::POLYGON_MODE_LINE,
        required_limits: wgpu::Limits::default(),
    };
    // Tracing is disabled
    adapter.request_device(desc, None).await.unwrap()
}

async fn adapter(instance: &wgpu::Instance) -> wgpu::Adapter {
    let options = &wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
    };
    instance.request_adapter(options).await.unwrap()
}
