pub mod mesh;
pub mod texture;

use mesh::Mesh;
use texture::Texture;

pub struct Item {
    pub mesh: Mesh,
    pub texture: Texture,
}

impl Item {
    pub fn new(mesh: Mesh, texture: Texture) -> Self {
        Self { mesh, texture }
    }
}
