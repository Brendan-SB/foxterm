use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{ErrorKind, Read, Write},
    path::Path,
};

pub const DEFAULT_CONFIG_DIR: &str = "~/.config/foxterm/config.yaml";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Font {
    pub path: String,
    pub color: [f32; 4],
    pub scale: f32,
}

impl Font {
    pub fn new(path: String, color: [f32; 4], scale: f32) -> Self {
        Self { path, color, scale }
    }
}

impl Default for Font {
    fn default() -> Self {
        Self::new("test.ttf".to_owned(), [1.0; 4], 40.0)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub device_index: Option<usize>,
    pub bg_color: [f32; 4],
    pub font: Font,
}

impl Config {
    pub fn new(device_index: Option<usize>, bg_color: [f32; 4], font: Font) -> Self {
        Self {
            device_index,
            bg_color,
            font,
        }
    }

    pub fn from_file(path: &String) -> anyhow::Result<Self> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                if let ErrorKind::NotFound = e.kind() {
                    let config = Self::default();

                    config.create_file()?;

                    return Ok(config);
                }

                return Err(e.into());
            }
        };
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        let config = serde_yaml::from_str(contents.as_str())?;

        Ok(config)
    }

    pub fn default_from_file() -> anyhow::Result<Self> {
        Self::from_file(&shellexpand::tilde(DEFAULT_CONFIG_DIR).as_ref().to_string())
    }

    pub fn create_file(&self) -> anyhow::Result<()> {
        let path = shellexpand::tilde(DEFAULT_CONFIG_DIR);
        let path = Path::new(path.as_ref());
        let config_dir = path.parent().unwrap();

        fs::create_dir_all(config_dir)?;

        let mut file = File::create(path)?;

        file.write_all(serde_yaml::to_string(self)?.as_bytes())?;

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new(None, [0.0; 4], Default::default())
    }
}
