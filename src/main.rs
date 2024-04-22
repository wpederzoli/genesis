use engine::Engine;
mod engine;

//TODO: clear resources in event loop run (verify)
//TODO: windowed and full-screen feature
//TODO: draw to screen
//TODO: on input

fn main() {
    Engine::new("Genesis")
        .with_clear_color(wgpu::Color::BLACK)
        .run();
}
