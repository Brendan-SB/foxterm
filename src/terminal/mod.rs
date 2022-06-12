pub mod config;
pub mod pty;

use config::Config;
use pty::Pty;
use std::{env, sync::Arc};
use winit_input_helper::{TextChar, WinitInputHelper};

pub struct Terminal {
    pub pty: Arc<Pty>,
    pub config: Config,
}

impl Terminal {
    pub fn new(pty: Arc<Pty>, config: Config) -> Self {
        Self { pty, config }
    }

    pub fn init() -> anyhow::Result<Option<Self>> {
        match Pty::spawn(env::var("SHELL").unwrap().to_owned())? {
            Some(pty) => {
                let config = Config::new([0.0; 4], "test.ttf".to_owned(), [1.0; 4], 40.0);

                Ok(Some(Self::new(Arc::new(pty), config)))
            }

            None => Ok(None),
        }
    }

    pub fn update_pty(&self, input: &WinitInputHelper) -> anyhow::Result<()> {
        let text = input
            .text()
            .iter()
            .map(|c| match c {
                TextChar::Char(c) => *c,
                TextChar::Back => '\u{8}',
            })
            .collect();

        self.pty.write(text)?;

        Ok(())
    }
}
