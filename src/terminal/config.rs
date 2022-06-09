pub struct Config {
    pub bg_color: [f32; 4],
    pub font_color: [f32; 4],
    pub font_scale: f32,
}

impl Config {
    pub fn new(bg_color: [f32; 4], font_color: [f32; 4], font_scale: f32) -> Self {
        Self {
            bg_color,
            font_color,
            font_scale,
        }
    }
}
