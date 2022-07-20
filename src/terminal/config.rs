use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read, Write},
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
        match Self::load_contents(path) {
            Ok(contents) => {
                let config = serde_yaml::from_str(contents.as_str())?;

                Ok(config)
            }
            Err(e) => match e.downcast_ref::<io::Error>().and_then(|e| Some(e.kind())) {
                Some(ErrorKind::NotFound) => {
                    let config = Self::default();

                    config.create_file(&DEFAULT_CONFIG_DIR.to_string())?;

                    Ok(config)
                }
                _ => Err(e.into()),
            },
        }
    }

    pub fn default_from_file() -> anyhow::Result<Self> {
        Self::from_file(&shellexpand::tilde(DEFAULT_CONFIG_DIR).as_ref().to_string())
    }

    fn create_file(&self, path: &String) -> anyhow::Result<()> {
        let path = shellexpand::tilde(path);
        let path = Path::new(path.as_ref());

        fs::create_dir_all(path.parent().unwrap())?;

        let mut file = File::create(path)?;

        file.write_all(serde_yaml::to_string(self)?.as_bytes())?;

        Ok(())
    }

    fn load_contents(path: &String) -> anyhow::Result<String> {
        let mut file = File::open(path)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        Ok(contents)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new(None, [0.0; 4], Default::default())
    }
}
