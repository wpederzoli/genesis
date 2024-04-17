use wgpu::{Device, DeviceDescriptor, Instance, InstanceDescriptor, Queue, RequestAdapterOptions};
use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

//TODO: run function to render some clera color

pub struct Engine {
    event_loop: EventLoop<()>,
    window: Window,
    device: Device,
    queue: Queue,
}

impl Engine {
    pub fn new(title: &str) -> Self {
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new()
            .with_title(title)
            .build(&event_loop)
            .unwrap();

        event_loop.set_control_flow(ControlFlow::Poll);
        let (device, queue) = pollster::block_on(Self::setup_wgpu(&window)).unwrap();

        Engine {
            event_loop,
            window,
            device,
            queue,
        }
    }

    pub fn run(&self) {
        todo!()
    }

    async fn setup_wgpu(window: &Window) -> Option<(Device, Queue)> {
        let instance = Instance::new(InstanceDescriptor::default());
        let surface = instance.create_surface(window).unwrap();
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        Some((device, queue))
    }
}
