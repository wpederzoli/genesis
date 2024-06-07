use std::collections::HashMap;

use log::info;

use crate::engine::{graphics::Graphics, scene::Scene};

///SceneManager handles adding, getting and switching scenes
///Scenes are stored in a HashMap <Label, Scene>
///We keep two references, one for the active scene and
///another for a `next_scene` which will indicate that we need
///to swap scenes.
pub struct SceneManager {
    scenes: HashMap<String, Box<dyn Scene>>,
    active_scene: Option<String>,
    next_scene: Option<String>,
}

impl SceneManager {
    ///Creates a new
    ///```
    ///SceneManager {
    /// scenes: HashMap::new(),
    /// active_scene: None,
    /// next_scene: None
    ///}
    ///```
    pub fn new() -> Self {
        info!("Scene manager initialized");
        SceneManager {
            scenes: HashMap::new(),
            active_scene: None,
            next_scene: None,
        }
    }

    ///Add a scene to the scenes hashmap
    pub fn add_scene(&mut self, label: &str, scene: Box<dyn Scene>) {
        info!("New scene added: {}", label);
        self.scenes.insert(label.to_string(), scene);
    }

    ///Pass a label of an existing scene, if the scene exists it will
    ///populate the `next_scene` property which will be handled in the
    ///`update` function by calling the `cleanup` for the `active_scene`
    ///and setting the requested scene as the active_scene
    pub fn request_scene_change(&mut self, label: &str) {
        info!("Scene change requested: {}", label);
        if self.scenes.contains_key(label) {
            self.next_scene = Some(label.to_string());
        } else {
            info!("Requested scene not found: {}", label);
        }
    }

    ///Checks if there is a `next_scene` in line, if there is, it calls the
    ///`cleanup()` function for the `active_scene`, sets the `next_scene` as the new `active_scene`
    ///clears the `next_scene` property andinitializes the new `active_scene` if necessary
    pub fn update(&mut self, graphics: &mut Graphics) {
        if let Some(ref next_scene_label) = self.next_scene {
            if let Some(ref current_scene_label) = self.active_scene {
                if let Some(scene) = self.scenes.get_mut(current_scene_label) {
                    info!("Clearing up active scene: {}", current_scene_label);
                    scene.cleanup();
                }
            }

            self.active_scene = Some(next_scene_label.clone());
            if let Some(scene) = self.scenes.get_mut(next_scene_label) {
                if !scene.is_initialized() {
                    info!("Initializing next scene: {}", next_scene_label);
                    scene.init(graphics);
                }
            }

            self.next_scene = None;
        }
    }

    ///Retunrs the currently active scene
    pub fn get_active_scene(&mut self) -> Option<&mut Box<dyn Scene>> {
        info!("Active scene requested");
        if let Some(ref label) = self.active_scene {
            self.scenes.get_mut(label)
        } else {
            info!("No active scene found");
            None
        }
    }
}
