use std::sync::Arc;
use vulkano::{
    device::Queue,
    format::Format,
    image::{view::ImageView, ImageDimensions, ImmutableImage, MipmapsCount},
    sampler::Sampler,
};

pub struct Texture {
    pub image: Arc<ImageView<ImmutableImage>>,
    pub sampler: Arc<Sampler>,
}

impl Texture {
    pub fn new(
        queue: Arc<Queue>,
        sampler: Arc<Sampler>,
        format: Format,
        dimensions: ImageDimensions,
        bytes: &[u8],
    ) -> anyhow::Result<Self> {
        let (image, _) = ImmutableImage::from_iter(
            bytes.iter().cloned(),
            dimensions,
            MipmapsCount::One,
            format,
            queue.clone(),
        )?;
        let image = ImageView::new_default(image)?;

        Ok(Self { image, sampler })
    }
}
