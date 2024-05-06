use winit::{
    event_loop::{self, EventLoop},
    window::WindowBuilder,
};

pub struct Window {
    pub window: winit::window::Window,
    pub event_loop: winit::event_loop::EventLoop<()>,
}

impl Window {
    pub fn new(title: &str) -> Self {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(event_loop::ControlFlow::Poll);
        let window = WindowBuilder::new()
            .with_title(title)
            .build(&event_loop)
            .unwrap();

        Window { window, event_loop }
    }
}
