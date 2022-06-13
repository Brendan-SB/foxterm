use crate::{
    mesh::{Mesh, Vertex},
    texture::Texture,
    SCALE,
};
use cgmath::Vector2;
use fontdue::Metrics;
use std::sync::Arc;
use vulkano::{device::Device, device::Queue, format::Format, image::ImageDimensions};

pub struct Chr {
    pub dimensions: Vector2<f32>,
    pub bearing: Vector2<f32>,
    pub mesh: Mesh,
    pub texture: Texture,
}

impl Chr {
    pub fn new(
        dimensions: Vector2<f32>,
        bearing: Vector2<f32>,
        mesh: Mesh,
        texture: Texture,
    ) -> Self {
        Self {
            dimensions,
            bearing,
            mesh,
            texture,
        }
    }

    pub fn from_bitmap(
        device: Arc<Device>,
        queue: Arc<Queue>,
        metrics: &Metrics,
        bitmap: &Vec<u8>,
    ) -> anyhow::Result<Self> {
        let dimensions = Vector2::new(metrics.width as f32, metrics.height as f32) * SCALE;
        let bearing = Vector2::new(metrics.xmin as f32, metrics.ymin as f32) * SCALE;
        let mesh = Self::create_mesh(device.clone(), dimensions)?;
        let texture = Self::create_texture(device.clone(), queue, metrics, bitmap)?;

        Ok(Self::new(dimensions, bearing, mesh, texture))
    }

    fn create_mesh(device: Arc<Device>, dimensions: Vector2<f32>) -> anyhow::Result<Mesh> {
        let vertices = {
            [
                Vertex {
                    uv: [0.0, 0.0],
                    position: [0.0, 0.0, 0.0],
                },
                Vertex {
                    uv: [0.0, 1.0],
                    position: [0.0, dimensions.y, 0.0],
                },
                Vertex {
                    uv: [1.0, 0.0],
                    position: [dimensions.x, 0.0, 0.0],
                },
                Vertex {
                    uv: [1.0, 1.0],
                    position: [dimensions.x, dimensions.y, 0.0],
                },
            ]
        };
        let indices = [0, 1, 2, 1, 2, 3];
        let mesh = Mesh::new(device.clone(), &vertices, &indices)?;

        Ok(mesh)
    }

    fn create_texture(
        device: Arc<Device>,
        queue: Arc<Queue>,
        metrics: &Metrics,
        bitmap: &Vec<u8>,
    ) -> anyhow::Result<Texture> {
        let dims = ImageDimensions::Dim2d {
            width: metrics.width as u32,
            height: metrics.height as u32,
            array_layers: 1,
        };
        let texture = Texture::new(device, queue, Format::R8_SRGB, dims, bitmap)?;

        Ok(texture)
    }
}
