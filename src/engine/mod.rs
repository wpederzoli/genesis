use log::error;
use wgpu::{
    Device, DeviceDescriptor, Instance, InstanceDescriptor, Queue, RequestAdapterOptions, Surface,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub struct Engine {
    event_loop: Option<EventLoop<()>>,
    device: Option<Device>,
    queue: Option<Queue>,
    surface: Option<Surface<'static>>,
}

impl Engine {
    pub fn new(title: &str) -> Self {
        let mut engine = Engine::default();

        match EventLoop::new() {
            Ok(el) => engine.event_loop = Some(el),
            Err(e) => {
                error!("Failed to create event loop: {}", e);
                return Engine::default();
            }
        };

        let window: Window;
        match WindowBuilder::new()
            .with_title(title)
            .build(&engine.event_loop.as_ref().unwrap())
        {
            Ok(win) => window = win,
            Err(e) => {
                error!("Failed to create window: {}", e);
                return Engine::default();
            }
        }

        engine
            .event_loop
            .as_ref()
            .unwrap()
            .set_control_flow(ControlFlow::Poll);

        let size = window.inner_size();
        let instance = Instance::new(InstanceDescriptor::default());

        match instance.create_surface(window) {
            Ok(surface) => engine.surface = Some(surface),
            Err(e) => {
                error!("Failed to create surface: {}", e);
                return Engine::default();
            }
        }

        let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&engine.surface.as_ref().unwrap()),
        }))
        .unwrap();

        match pollster::block_on(adapter.request_device(
            &DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits:
                    wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits()),
            },
            None,
        )) {
            Ok((device, queue)) => {
                engine.device = Some(device);
                engine.queue = Some(queue);
            }
            Err(e) => {
                error!("Unable to create device and queue: {}", e);
                return Engine::default();
            }
        }

        let config = engine
            .surface
            .as_ref()
            .unwrap()
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        engine
            .surface
            .as_ref()
            .unwrap()
            .configure(&engine.device.as_ref().unwrap(), &config);

        engine
    }

    pub fn run(self) {
        match self.event_loop {
            Some(ev) => {
                let Engine {
                    queue,
                    surface,
                    device,
                    ..
                } = self;
                let surface = surface.unwrap();
                let queue = queue.unwrap();
                let device = device.unwrap();

                ev.run(move |event, target| match event {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
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
                        let mut encoder =
                            device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                label: None,
                            });

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
            None => {
                error!("Event loop is not set!");
            }
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Engine {
            event_loop: None,
            device: None,
            queue: None,
            surface: None,
        }
    }
}
