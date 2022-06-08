vulkano_shaders::shader! {
    ty: "vertex",
    path: "src/shaders/vertex.glsl",
    types_meta: {
        use bytemuck::{Pod, Zeroable};

        #[derive(Clone, Copy, Zeroable, Pod)]
    }
}
