///This struct holds the configurable options of the window
///They can be queried and updated
pub struct Config {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
}

impl Config {
    pub fn new(title: &str, width: u32, height: u32, fullscreen: bool) -> Self {
        Config {
            title: title.to_string(),
            width,
            height,
            fullscreen,
        }
    }
}
