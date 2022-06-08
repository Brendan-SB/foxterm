pub struct Config {
    pub bg_color: [f32; 4],
    pub text_color: [f32; 4],
}

impl Config {
    pub fn new(bg_color: [f32; 4], text_color: [f32; 4]) -> Self {
        Self {
            bg_color,
            text_color,
        }
    }
}
