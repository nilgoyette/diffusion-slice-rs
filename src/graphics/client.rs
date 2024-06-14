use super::*;

/// Stores handlers related to the user environment
pub struct Client {
    pub device: Device,
    pub command_queue: Queue,
    pub img_size: (u32, u32),
    pub output_path: PathBuf,
}

impl Client {
    pub async fn new(inputs: &UserInputs) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

        let (device, command_queue) = {
            let adapter = adapter(&instance).await;
            device(&adapter).await
        };
        Self {
            device,
            command_queue,
            img_size: inputs.dst_img_size,
            output_path: inputs.dst_img_path.clone(),
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
