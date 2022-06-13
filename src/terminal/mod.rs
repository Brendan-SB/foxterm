pub mod config;
pub mod pty;

use config::Config;
use pty::Pty;
use std::{
    env,
    sync::{Arc, RwLock},
    thread,
};
use winit_input_helper::{TextChar, WinitInputHelper};

pub struct Terminal {
    pub config: Config,
    pub pty: Arc<Pty>,
    pub buf: Arc<RwLock<String>>,
}

impl Terminal {
    pub fn new(config: Config, pty: Arc<Pty>, buf: Arc<RwLock<String>>) -> Self {
        Self { config, pty, buf }
    }

    pub fn init() -> anyhow::Result<Option<Self>> {
        match Pty::spawn(env::var("SHELL").unwrap().to_owned())? {
            Some(pty) => {
                let config = Config::new([0.0; 4], "test.ttf".to_owned(), [1.0; 4], 40.0);
                let pty = Arc::new(pty);
                let buf = Arc::new(RwLock::new(String::new()));

                Self::spawn_worker(pty.clone(), buf.clone());

                Ok(Some(Self::new(config, pty, buf)))
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

    pub fn spawn_worker(pty: Arc<Pty>, buf: Arc<RwLock<String>>) {
        thread::spawn(move || loop {
            match pty.read() {
                Ok(content) => *buf.write().unwrap() += content.as_str(),
                Err(_) => break,
            }
        });
    }
}
