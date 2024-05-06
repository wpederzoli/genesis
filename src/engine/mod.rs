use winit::event::{Event, WindowEvent};

use self::{
    graphics::Graphics,
    scene::{BaseScene, Scene},
    scene_manager::scene_manager::SceneManager,
    window::Window,
};

pub mod graphics;
pub mod scene;
mod scene_manager;
mod window;

pub struct Engine {
    window: Window,
    scene_manager: SceneManager,
}

impl Engine {
    pub fn new(title: &str) -> Self {
        let window = Window::new(title);
        let scene_manager = SceneManager::new();

        Engine {
            window,
            scene_manager,
        }
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

    pub fn run(self) {
        let mut graphics = Graphics::new(&self.window.window);

        self.window
            .event_loop
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
                    let frame = graphics
                        .surface
                        .get_current_texture()
                        .expect("Failed to get texture");

                    let view = frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());

                    let mut encoder = graphics
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(graphics.clear_color),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });

                    graphics.queue.submit(Some(encoder.finish()));
                    frame.present();

                    if let Some(scene) = self.scene_manager.get_active_scene() {
                        scene.draw(&mut graphics);
                    }
                }

                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    graphics.config.width = size.width;
                    graphics.config.height = size.height;
                    graphics
                        .surface
                        .configure(&graphics.device, &graphics.config);
                }
                _ => (),
            })
            .unwrap();
    }
}
