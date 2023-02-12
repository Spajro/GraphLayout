#[derive(Copy, Clone)]
pub struct Config {
    pub iterations: u32,
    pub image_width: u32,
    pub image_height: u32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            iterations: 100,
            image_width: 1920,
            image_height: 1080,
        }
    }
}

impl Config {
    pub fn draw_height(&self) -> u32 {
        self.image_height - 200
    }
    pub fn draw_width(&self) -> u32 {
        self.image_width - 200
    }
}