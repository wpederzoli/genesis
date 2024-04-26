use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
};

use self::{
    graphics::Graphics,
    scene::{BaseScene, Scene},
    scene_manager::scene_manager::SceneManager,
};

pub mod graphics;
pub mod scene;
mod scene_manager;

pub struct Engine {
    event_loop: EventLoop<()>,
    graphics: Graphics,
    scene_manager: SceneManager,
}

impl Engine {
    pub fn new(title: &str) -> Self {
        let event_loop = EventLoop::new().unwrap();
        let graphics = Graphics::new(title, &event_loop);
        let scene_manager = SceneManager::new();

        Engine {
            event_loop,
            graphics,
            scene_manager,
        }
    }

    pub fn with_clear_color(mut self, color: wgpu::Color) -> Self {
        self.graphics.set_clear_color(color);
        Engine { ..self }
    }

    pub fn add_scene(mut self, scene: Scene) -> Self {
        self.scene_manager.add_scene(scene);
        Engine { ..self }
    }

    pub fn add_active_scene(mut self, scene: Scene) -> Self {
        let label = scene.label.clone();
        self.scene_manager.add_scene(scene);
        self.scene_manager.set_active_scene(&label);

        Engine { ..self }
    }

    pub fn run(mut self) {
        self.event_loop
            .run(move |event, target| match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => target.exit(),

                Event::WindowEvent {
                    event: WindowEvent::KeyboardInput { event, .. },
                    ..
                } => {
                    if let Some(scene) = self.scene_manager.get_active_scene() {
                        scene.input(&event.physical_key, &target);
                    }
                }
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

                    self.graphics.queue.submit(Some(encoder.finish()));
                    frame.present();

                    if let Some(scene) = self.scene_manager.get_active_scene() {
                        scene.draw(&mut self.graphics);
                    }
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
