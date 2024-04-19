use engine::Engine;

mod engine;

//TODO: add logging
//TODO: resize
//TODO: refactor

fn main() {
    pollster::block_on(Engine::new("Genesis")).run();
}
