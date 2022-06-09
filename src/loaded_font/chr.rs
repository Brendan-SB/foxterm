use crate::{
    mesh::{Mesh, Vertex},
    texture::Texture,
};
use fontdue::Metrics;
use std::{rc::Rc, sync::Arc};
use vulkano::{device::Device, device::Queue, format::Format, image::ImageDimensions};

pub struct Chr {
    pub mesh: Mesh,
    pub texture: Texture,
}

impl Chr {
    pub fn new(mesh: Mesh, texture: Texture) -> Rc<Self> {
        Rc::new(Self { mesh, texture })
    }

    pub fn from_bitmap(
        device: Arc<Device>,
        queue: Arc<Queue>,
        metrics: &Metrics,
        bitmap: &Vec<u8>,
    ) -> anyhow::Result<Rc<Self>> {
        let mesh = Self::create_mesh(device.clone(), metrics)?;
        let texture = Self::create_texture(device.clone(), queue, metrics, bitmap)?;

        Ok(Self::new(mesh, texture))
    }

    fn create_mesh(device: Arc<Device>, metrics: &Metrics) -> anyhow::Result<Mesh> {
        let width = metrics.width as f32 / 100.0;
        let height = metrics.height as f32 / 100.0;
        let vertices = {
            let width = width / 2.0;
            let height = height / 2.0;

            [
                Vertex {
                    uv: [0.0, 0.0],
                    position: [-width, -height, 0.0],
                },
                Vertex {
                    uv: [0.0, 1.0],
                    position: [-width, height, 0.0],
                },
                Vertex {
                    uv: [1.0, 0.0],
                    position: [width, -height, 0.0],
                },
                Vertex {
                    uv: [1.0, 1.0],
                    position: [width, height, 0.0],
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
