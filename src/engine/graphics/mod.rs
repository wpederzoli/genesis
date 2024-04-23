use wgpu::{Adapter, Device, DeviceDescriptor, Queue, Surface, SurfaceConfiguration};
use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const DEFAULT_CLEAR_COLOR: wgpu::Color = wgpu::Color {
    r: 0.3,
    g: 0.5,
    b: 0.4,
    a: 1.0,
};

pub struct Graphics {
    pub device: Device,
    pub queue: Queue,
    pub adapter: Adapter,
    pub surface: Surface<'static>,
    pub config: SurfaceConfiguration,
    pub clear_color: wgpu::Color,
}

impl Graphics {
    pub fn new(title: &str, event_loop: &EventLoop<()>) -> Self {
        let window = WindowBuilder::new()
            .with_title(title)
            .build(&event_loop)
            .unwrap();

        event_loop.set_control_flow(ControlFlow::Poll);

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));

        let size = window.inner_size();
        let surface = instance.create_surface(window).unwrap();

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                label: None,
            },
            None,
        ))
        .unwrap();

        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();

        surface.configure(&device, &config);

        Graphics {
            device,
            queue,
            adapter,
            surface,
            config,
            clear_color: DEFAULT_CLEAR_COLOR,
        }
    }

    pub fn set_clear_color(&mut self, color: wgpu::Color) {
        self.clear_color = color;
    }
}
