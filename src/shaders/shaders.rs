use super::{fragment, vertex};
use std::sync::Arc;
use vulkano::{device::Device, shader::ShaderModule};

pub struct Shaders {
    pub vertex: Arc<ShaderModule>,
    pub fragment: Arc<ShaderModule>,
}

impl Shaders {
    pub fn new(device: Arc<Device>) -> anyhow::Result<Arc<Self>> {
        Ok(Arc::new(Self {
            vertex: vertex::load(device.clone())?,
            fragment: fragment::load(device.clone())?,
        }))
    }
}
