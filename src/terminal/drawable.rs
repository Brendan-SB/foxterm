use crate::loaded_font::chr::Chr;
use cgmath::Vector2;
use std::sync::Arc;

pub struct Drawable {
    pub render_item: RenderItem,
    pub pos: Vector2<f32>,
}

impl Drawable {
    pub fn new(render_item: RenderItem, pos: Vector2<f32>) -> Self {
        Self { render_item, pos }
    }
}

pub enum RenderItem {
    Chr(Arc<Chr>),
    Space,
}
