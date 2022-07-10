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
    pub fn new(image: Arc<ImageView<ImmutableImage>>, sampler: Arc<Sampler>) -> Self {
        Self { image, sampler }
    }

    pub fn from_data(
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

        Ok(Self::new(image, sampler))
    }

    pub fn white(device: Arc<Device>, queue: Arc<Queue>) -> anyhow::Result<Self> {
        Self::from_data(
            device,
            queue,
            Format::R8_UNORM,
            ImageDimensions::Dim2d {
                width: 1,
                height: 1,
                array_layers: 1,
            },
            &[u8::MAX],
        )
    }
}
