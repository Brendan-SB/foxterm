pub mod chr;

use crate::{terminal::config::Config, SCALE};
use chr::Chr;
use fontdue::{Font, FontSettings};
use std::{fs::File, io::Read, sync::Arc};
use thiserror::Error;
use vulkano::{device::Device, device::Queue};

pub struct LoadedFont {
    pub chrs: Vec<Arc<Chr>>,
    pub scale: f32,
}

impl LoadedFont {
    pub fn new(chrs: Vec<Arc<Chr>>, scale: f32) -> Self {
        Self { chrs, scale }
    }

    pub fn from_file(
        device: Arc<Device>,
        queue: Arc<Queue>,
        config: &Config,
    ) -> anyhow::Result<Self> {
        let bytes = Self::load_bytes(&config.font.path)?;
        let font = Self::try_font_from_fontdue_result(Font::from_bytes(
            bytes.as_slice(),
            FontSettings::default(),
        ))?;
        let chrs = Self::create_chrs(device, queue, &font, config.font.scale);

        Ok(Self::new(chrs, config.font.scale * SCALE))
    }

    pub fn get_chr_by_id(&self, id: u8) -> Option<Arc<Chr>> {
        if id >= 33 {
            let i = id as usize - 33;

            self.chrs.get(i).cloned()
        } else {
            None
        }
    }

    fn load_bytes(path: &String) -> anyhow::Result<Vec<u8>> {
        let mut file = File::open(shellexpand::tilde(path).as_ref())?;
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)?;

        Ok(buffer)
    }

    fn try_font_from_fontdue_result(e: Result<Font, &'static str>) -> anyhow::Result<Font> {
        match e {
            Ok(f) => Ok(f),
            Err(e) => Err(LoadedFontError::StrError(e).into()),
        }
    }

    fn create_chrs(
        device: Arc<Device>,
        queue: Arc<Queue>,
        font: &Font,
        scale: f32,
    ) -> Vec<Arc<Chr>> {
        (33..=126_u8)
            .filter_map(|i| {
                let c = i as char;
                let (metrics, bitmap) = font.rasterize(c, scale);

                match Chr::from_bitmap(i, device.clone(), queue.clone(), &metrics, &bitmap) {
                    Ok(chr) => Some(Arc::new(chr)),
                    Err(_) => None,
                }
            })
            .collect()
    }
}

impl Default for LoadedFont {
    fn default() -> Self {
        Self::new(Vec::new(), 0.0)
    }
}

#[derive(Debug, Error)]
pub enum LoadedFontError {
    #[error("Error: {0}")]
    StrError(&'static str),
}
