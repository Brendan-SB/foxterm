use crate::loaded_font::chr::Chr;
use cgmath::Vector2;
use std::sync::Arc;

pub struct Drawable {
    pub chr: Arc<Chr>,
    pub pos: Vector2<f32>,
}

impl Drawable {
    pub fn new(chr: Arc<Chr>, pos: Vector2<f32>) -> Self {
        Self { chr, pos }
    }
}
