pub mod chr;

use crate::terminal::config::Config;
use chr::Chr;
use fontdue::{Font, FontSettings};
use std::{fs::File, io::Read, rc::Rc, sync::Arc};
use thiserror::Error;
use vulkano::{device::Device, device::Queue};

pub struct LoadedFont {
    pub chrs: Vec<Rc<Chr>>,
}

impl LoadedFont {
    pub fn new(chrs: Vec<Rc<Chr>>) -> Self {
        Self { chrs }
    }

    pub fn from_file(
        device: Arc<Device>,
        queue: Arc<Queue>,
        config: &Config,
        path: &String,
    ) -> anyhow::Result<Self> {
        let bytes = Self::load_bytes(path)?;
        let font = match Font::from_bytes(bytes.as_slice(), FontSettings::default()) {
            Ok(font) => font,
            Err(e) => return Err(LoadedFontError::FontdueError(e).into()),
        };
        let chrs = Self::create_chrs(device, queue, &font, config.font_scale);

        Ok(Self::new(chrs))
    }

    pub fn get_chr_by_id(&self, id: char) -> Option<Rc<Chr>> {
        if id >= 33 as char {
            let i = id as usize - 33;

            match self.chrs.get(i) {
                Some(chr) => Some(chr.clone()),
                None => None,
            }
        } else {
            None
        }
    }

    fn load_bytes(path: &String) -> anyhow::Result<Vec<u8>> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)?;

        Ok(buffer)
    }

    fn create_chrs(
        device: Arc<Device>,
        queue: Arc<Queue>,
        font: &Font,
        scale: f32,
    ) -> Vec<Rc<Chr>> {
        (33..=126_u8)
            .filter_map(|i| {
                let c = i as char;
                let (metrics, bitmap) = font.rasterize(c, scale);

                match Chr::from_bitmap(device.clone(), queue.clone(), &metrics, &bitmap) {
                    Ok(chr) => Some(Rc::new(chr)),
                    Err(_) => None,
                }
            })
            .collect()
    }
}

#[derive(Debug, Error)]
pub enum LoadedFontError {
    #[error("Fontdue error: {0}")]
    FontdueError(&'static str),
}
