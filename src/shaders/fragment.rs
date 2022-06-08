vulkano_shaders::shader! {
    ty: "fragment",
    path: "src/shaders/fragment.glsl",
    types_meta: {
        use bytemuck::{Pod, Zeroable};

        #[derive(Clone, Copy, Zeroable, Pod)]
    }
}
