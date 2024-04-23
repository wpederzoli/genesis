use std::sync::Arc;

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoopWindowTarget,
    keyboard::PhysicalKey,
};

use self::graphics::Graphics;

mod graphics;

pub struct Engine {
    graphics: Graphics,
    input_handler: Arc<dyn Fn(&PhysicalKey, &EventLoopWindowTarget<()>)>,
}

impl Engine {
    pub fn new(title: &str) -> Self {
        let graphics = Graphics::new(title);

        Engine {
            graphics,
            input_handler: Arc::new(|_, _| {}),
        }
    }

    pub fn with_clear_color(mut self, color: wgpu::Color) -> Self {
        self.graphics.set_clear_color(color);
        Engine { ..self }
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

        self.graphics
            .event_loop
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
                        .graphics
                        .surface
                        .get_current_texture()
                        .expect("Failed to get texture");
                    let view = frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());
                    let mut encoder = self
                        .graphics
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(self.graphics.clear_color),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });

                    //Add client's drawing logic

                    self.graphics.queue.submit(Some(encoder.finish()));
                    frame.present();
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    self.graphics.config.width = size.width;
                    self.graphics.config.height = size.height;
                    self.graphics
                        .surface
                        .configure(&self.graphics.device, &self.graphics.config);
                }
                _ => (),
            })
            .unwrap();
    }
}
