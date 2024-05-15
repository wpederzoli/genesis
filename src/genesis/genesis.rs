use std::sync::Arc;

use crate::engine::scene::Scene;

use super::scenes::gameplay::GamePlay;

pub enum GameState {
    GamePlay,
}

//TODO: Revisit this approach
impl GameState {
    pub fn init(&self) -> Scene {
        match self {
            GameState::GamePlay => {
                let game_play = Arc::new(GamePlay::new());
                let gameplay_clone = Arc::clone(&game_play);
                let g_copy = Arc::clone(&gameplay_clone);

                Scene::new("GamePlay")
                    .with_init_system(move |graphics| g_copy.init(graphics))
                    .with_input_system(move |key, target| gameplay_clone.input(key, target))
                    .with_draw_system(move |graphics| game_play.draw(graphics))
            }
        }
    }
}
