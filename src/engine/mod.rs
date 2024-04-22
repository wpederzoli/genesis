use std::sync::Arc;

use wgpu::{Device, DeviceDescriptor, Queue, Surface, SurfaceConfiguration};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    keyboard::PhysicalKey,
    window::WindowBuilder,
};

mod graphics;

const DEFAULT_CLEAR_COLOR: wgpu::Color = wgpu::Color {
    r: 0.3,
    g: 0.5,
    b: 0.4,
    a: 1.0,
};

//TODO: Abstract graphics
pub struct Engine {
    event_loop: EventLoop<()>,
    device: Device,
    queue: Queue,
    surface: Surface<'static>,
    config: SurfaceConfiguration,
    clear_color: wgpu::Color,
    input_handler: Arc<dyn Fn(&PhysicalKey, &EventLoopWindowTarget<()>)>,
}

impl Engine {
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
            input_handler: Arc::new(|_, _| {}),
        }
    }

    pub fn with_clear_color(self, color: wgpu::Color) -> Self {
        Engine {
            clear_color: color,
            ..self
        }
    }

    pub fn with_input<F>(self, input_handler: F) -> Self
    where
        F: Fn(&PhysicalKey, &EventLoopWindowTarget<()>) + 'static,
    {
        Engine {
            input_handler: Arc::new(input_handler),
            ..self
        }
    }

    pub fn run(mut self) {
        let input_handler = Arc::clone(&self.input_handler);

        self.event_loop
            .run(move |event, target| match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => target.exit(),
                Event::WindowEvent {
                    event: WindowEvent::KeyboardInput { event, .. },
                    ..
                } => input_handler(&event.physical_key, &target),
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
