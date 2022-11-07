use crate::{
    item::{mesh::Mesh, texture::Texture, Item},
    SCALE,
};
use cgmath::Vector2;
use fontdue::Metrics;
use std::sync::Arc;
use vulkano::{device::Device, device::Queue, format::Format, image::ImageDimensions};

pub struct Chr {
    pub id: u8,
    pub dimensions: Vector2<f32>,
    pub bearing: Vector2<f32>,
    pub item: Item,
}

impl Chr {
    pub fn new(id: u8, dimensions: Vector2<f32>, bearing: Vector2<f32>, item: Item) -> Self {
        Self {
            id,
            dimensions,
            bearing,
            item,
        }
    }

    pub fn from_bitmap(
        id: u8,
        device: Arc<Device>,
        queue: Arc<Queue>,
        metrics: &Metrics,
        bitmap: &[u8],
    ) -> anyhow::Result<Self> {
        let dimensions = Vector2::new(metrics.width as f32, metrics.height as f32) * SCALE;
        let bearing = Vector2::new(metrics.xmin as f32, metrics.ymin as f32) * SCALE;
        let mesh = Mesh::from_rect(queue.clone(), dimensions)?;
        let texture = Self::create_texture(device, queue, metrics, bitmap)?;
        let item = Item::new(mesh, texture);

        Ok(Self::new(id, dimensions, bearing, item))
    }

    fn create_texture(
        device: Arc<Device>,
        queue: Arc<Queue>,
        metrics: &Metrics,
        bitmap: &[u8],
    ) -> anyhow::Result<Texture> {
        let dims = ImageDimensions::Dim2d {
            width: metrics.width as u32,
            height: metrics.height as u32,
            array_layers: 1,
        };
        let texture = Texture::from_data(device, queue, Format::R8_SRGB, dims, bitmap)?;

        Ok(texture)
    }
}
