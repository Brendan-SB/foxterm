use std::sync::Arc;
use vulkano::{
    device::{Device, Queue},
    format::Format,
    image::{view::ImageView, ImageDimensions, ImmutableImage, MipmapsCount},
    sampler::{Filter, Sampler, SamplerAddressMode, SamplerCreateInfo},
};

pub struct Texture {
    pub image: Arc<ImageView<ImmutableImage>>,
    pub sampler: Arc<Sampler>,
}

impl Texture {
    pub fn new(
        device: Arc<Device>,
        queue: Arc<Queue>,
        format: Format,
        dimensions: ImageDimensions,
        data: &[u8],
    ) -> anyhow::Result<Self> {
        let sampler = Sampler::new(
            device.clone(),
            SamplerCreateInfo {
                mag_filter: Filter::Linear,
                min_filter: Filter::Linear,
                address_mode: [SamplerAddressMode::Repeat; 3],
                mip_lod_bias: 1.0,
                lod: 0.0..=100.0,
                ..Default::default()
            },
        )?;
        let (image, _) = ImmutableImage::from_iter(
            data.iter().cloned(),
            dimensions,
            MipmapsCount::One,
            format,
            queue.clone(),
        )?;
        let image = ImageView::new_default(image)?;

        Ok(Self { image, sampler })
    }
}
