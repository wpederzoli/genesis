use engine::Engine;

mod engine;

//TODO: esc key
//TODO: resize
//TODO: refactor
//TODO: clear resources in event loop run

fn main() {
    Engine::new("Genesis").run();
}
