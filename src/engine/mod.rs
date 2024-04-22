use wgpu::{Device, DeviceDescriptor, Queue, Surface, SurfaceConfiguration};
use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{self, KeyCode},
    window::WindowBuilder,
};

const DEFAULT_CLEAR_COLOR: wgpu::Color = wgpu::Color {
    r: 0.3,
    g: 0.5,
    b: 0.4,
    a: 1.0,
};

pub struct Engine {
    event_loop: EventLoop<()>,
    device: Device,
    queue: Queue,
    surface: Surface<'static>,
    config: SurfaceConfiguration,
    clear_color: wgpu::Color,
}

impl<'a> Engine {
    pub fn new(title: &str) -> Self {
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new()
            .with_title(title)
            .build(&event_loop)
            .unwrap();

        event_loop.set_control_flow(ControlFlow::Poll);

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));

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

        Engine {
            event_loop,
            device,
            queue,
            surface,
            config,
            clear_color: DEFAULT_CLEAR_COLOR,
        }
    }

    pub fn with_clear_color(self, color: wgpu::Color) -> Self {
        Engine {
            event_loop: self.event_loop,
            surface: self.surface,
            device: self.device,
            queue: self.queue,
            config: self.config,
            clear_color: color,
        }
    }

    pub fn run(mut self) {
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
                    let frame = self
                        .surface
                        .get_current_texture()
                        .expect("Failed to get texture");
                    let view = frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());
                    let mut encoder = self
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(self.clear_color),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });

                    self.queue.submit(Some(encoder.finish()));
                    frame.present();
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    self.config.width = size.width;
                    self.config.height = size.height;
                    self.surface.configure(&self.device, &self.config);
                }
                _ => (),
            })
            .unwrap();
    }
}
