pub mod config;
pub mod pty;

use config::Config;
use pty::Pty;
use std::{
    env,
    sync::{Arc, RwLock},
};

pub struct Terminal {
    pub pty: Arc<RwLock<Pty>>,
    pub config: Config,
}

impl Terminal {
    pub fn new(pty: Arc<RwLock<Pty>>, config: Config) -> Self {
        Self { pty, config }
    }

    pub fn init() -> anyhow::Result<Option<Self>> {
        match Pty::spawn(env::var("SHELL").unwrap().to_owned())? {
            Some(pty) => {
                let config = Config::new([0.0; 4], [1.0; 4], 20.0);

                Ok(Some(Self::new(Arc::new(RwLock::new(pty)), config)))
            }

            None => Ok(None),
        }
    }
}
