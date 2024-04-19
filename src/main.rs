use engine::Engine;

mod engine;

//TODO: resize
//TODO: refactor
//TODO: clear resources in event loop run

fn main() {
    Engine::new("Genesis").run();
}
