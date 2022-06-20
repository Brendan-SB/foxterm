use bytemuck::{Pod, Zeroable};
use std::sync::Arc;
use vulkano::{
    buffer::{BufferUsage, ImmutableBuffer},
    device::Queue,
};

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Zeroable, Pod)]
pub struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
}

vulkano::impl_vertex!(Vertex, position, uv);

pub struct Mesh {
    pub vertices: Arc<ImmutableBuffer<[Vertex]>>,
    pub indices: Arc<ImmutableBuffer<[u32]>>,
}

impl Mesh {
    pub fn new(
        vertices: Arc<ImmutableBuffer<[Vertex]>>,
        indices: Arc<ImmutableBuffer<[u32]>>,
    ) -> Self {
        Self { vertices, indices }
    }

    pub fn from_data(
        queue: Arc<Queue>,
        vertices: &[Vertex],
        indices: &[u32],
    ) -> anyhow::Result<Self> {
        let (vertices, _) = ImmutableBuffer::from_iter(
            vertices.iter().cloned(),
            BufferUsage::vertex_buffer(),
            queue.clone(),
        )?;
        let (indices, _) = ImmutableBuffer::from_iter(
            indices.iter().cloned(),
            BufferUsage::index_buffer(),
            queue.clone(),
        )?;

        Ok(Self::new(vertices, indices))
    }
}
