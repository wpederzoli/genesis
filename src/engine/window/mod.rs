use winit::{
    dpi::{PhysicalSize, Size},
    event_loop::{self, EventLoop},
    window::{Fullscreen, WindowBuilder},
};

use super::config::Config;

pub struct Window {
    pub window: winit::window::Window,
    pub event_loop: winit::event_loop::EventLoop<()>,
}

impl Window {
    pub fn new(config: Config) -> Self {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(event_loop::ControlFlow::Poll);
        let mut window_builder = WindowBuilder::new()
            .with_title(config.title)
            .with_inner_size(Size::new(PhysicalSize::new(config.width, config.height)));

        if config.fullscreen {
            window_builder = window_builder.with_fullscreen(Some(Fullscreen::Borderless(None)));
        }

        let window = window_builder.build(&event_loop).unwrap();

        Window { window, event_loop }
    }
}

///This will create a Window with all the default values
impl Default for Window {
    fn default() -> Self {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(event_loop::ControlFlow::Poll);
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        Window { window, event_loop }
    }
}
