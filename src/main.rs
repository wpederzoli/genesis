use engine::Engine;

mod engine;

//TODO: resize
//TODO: refactor
//TODO: clear resources in event loop run
//TODO: fix lifetimes (prob use for<'a> syntax)

fn main() {
    Engine::new("Genesis").run();
}
