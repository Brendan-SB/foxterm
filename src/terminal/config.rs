pub struct Config {
    pub device_index: Option<usize>,
    pub bg_color: [f32; 4],
    pub font_path: String,
    pub font_color: [f32; 4],
    pub font_scale: f32,
}

impl Config {
    pub fn new(
        device_index: Option<usize>,
        bg_color: [f32; 4],
        font_path: String,
        font_color: [f32; 4],
        font_scale: f32,
    ) -> Self {
        Self {
            device_index,
            bg_color,
            font_path,
            font_color,
            font_scale,
        }
    }
}
