use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    window::Fullscreen,
};

use self::{
    camera::camera_controller::CameraController,
    config::Config,
    graphics::Graphics,
    scene::{BaseScene, Scene},
    scene_manager::scene_manager::SceneManager,
    window::Window,
};

pub mod camera;
pub mod config;
pub mod graphics;
pub mod scene;
mod scene_manager;
mod window;

pub struct Engine {
    window: Window,
    scene_manager: SceneManager,
}

impl Engine {
    pub fn new() -> Self {
        let window = Window::default();
        let scene_manager = SceneManager::new();

        Engine {
            window,
            scene_manager,
        }
    }

    pub fn new_from(title: &str, width: u32, height: u32, fullscreen: bool) -> Self {
        let config = Config::new(title, width, height, fullscreen);
        let window = Window::new(config);
        let scene_manager = SceneManager::new();

        Engine {
            window,
            scene_manager,
        }
    }

    pub fn with_title(self, title: &str) -> Self {
        self.window.window.set_title(title);

        Engine { ..self }
    }

    pub fn with_dimensions(self, width: u32, height: u32) -> Self {
        let _ = self
            .window
            .window
            .request_inner_size(PhysicalSize::new(width, height));

        Engine { ..self }
    }

    pub fn fullscreen(self) -> Self {
        self.window
            .window
            .set_fullscreen(Some(Fullscreen::Borderless(None)));

        Engine { ..self }
    }

    pub fn add_scene(mut self, scene: Scene) -> Self {
        self.scene_manager.add_scene(scene);
        Engine { ..self }
    }

    pub fn switch_scene(mut self, label: &str) -> Self {
        self.scene_manager.set_active_scene(label);

        Engine { ..self }
    }

    pub fn run(mut self) {
        let mut graphics = Graphics::new(&self.window.window);

        let mut camera_controller = CameraController::new(0.5);

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
                    camera_controller.process_events(&event);
                }
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    camera_controller.update_camera(&mut graphics.camera);
                    graphics.camera_uniform.update_view_proj(&graphics.camera);
                    graphics.queue.write_buffer(
                        &graphics.camera_buffer,
                        0,
                        bytemuck::cast_slice(&[graphics.camera_uniform]),
                    );

                    graphics.render();

                    if let Some(scene) = self.scene_manager.get_active_scene() {
                        if !scene.initialized {
                            scene.init(&mut graphics);
                        }

                        scene.draw(&mut graphics);
                    }

                    graphics.window.request_redraw();
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
