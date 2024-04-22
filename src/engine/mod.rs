use log::error;
use wgpu::{
    Device, DeviceDescriptor, Instance, InstanceDescriptor, Queue, RequestAdapterOptions, Surface,
    SurfaceConfiguration, TextureUsages,
};
use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{self, KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

pub struct Engine<'a> {
    event_loop: EventLoop<()>,
    device: Option<Device>,
    queue: Option<Queue>,
    surface: Option<Surface<'a>>,
    config: Option<SurfaceConfiguration>,
    window: Window,
}

impl<'a> Engine<'a> {
    pub fn new(title: &str) -> Self {
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new()
            .with_title(title)
            .build(&event_loop)
            .unwrap();

        Engine {
            event_loop,
            window,
            device: None,
            queue: None,
            surface: None,
            config: None,
        }
    }

    fn wgpu_setup(&'a mut self) {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(&self.window).unwrap();

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

        let size = self.window.inner_size();

        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();

        surface.configure(&device, &config);

        self.surface = Some(surface);
        self.config = Some(config);
        self.device = Some(device);
        self.queue = Some(queue);
    }

    pub fn run(mut self) {
        self.wgpu_setup();

        let surface = self.surface.unwrap();
        let device = self.device.unwrap();
        let queue = self.queue.unwrap();

        self.event_loop
            .run(move |event, target| match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => target.exit(),
                Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    physical_key: keyboard::PhysicalKey::Code(KeyCode::Escape),
                                    ..
                                },
                            ..
                        },
                    ..
                } => target.exit(),
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    let frame = surface
                        .get_current_texture()
                        .expect("Failed to get texture");
                    let view = frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());
                    let mut encoder = device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.3,
                                    g: 0.5,
                                    b: 0.4,
                                    a: 1.0,
                                }),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });

                    queue.submit(Some(encoder.finish()));
                    frame.present();
                }
                _ => (),
            })
            .unwrap();
    }
}
