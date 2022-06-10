pub mod fragment;
pub mod vertex;

use std::sync::Arc;
use vulkano::{device::Device, shader::ShaderModule};

pub struct Shaders {
    pub vertex: Arc<ShaderModule>,
    pub fragment: Arc<ShaderModule>,
}

impl Shaders {
    pub fn new(device: Arc<Device>) -> anyhow::Result<Self> {
        Ok(Self {
            vertex: vertex::load(device.clone())?,
            fragment: fragment::load(device.clone())?,
        })
    }
}
