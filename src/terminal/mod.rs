pub mod config;
pub mod pty;

use crate::loaded_font::{chr::Chr, LoadedFont};
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
    pub pty: Pty,
    pub buf: RwLock<Vec<Arc<Chr>>>,
}
impl Terminal {
    pub fn new(config: Config, pty: Pty, buf: RwLock<Vec<Arc<Chr>>>) -> Self {
        Self { config, pty, buf }
    }

    pub fn init() -> anyhow::Result<Option<Arc<Self>>> {
        match Pty::spawn(env::var("SHELL").unwrap().to_owned())? {
            Some(pty) => {
                let config = Config::new(Some(0), [0.0; 4], "test.ttf".to_owned(), [1.0; 4], 40.0);
                let pty = pty;
                let buf = RwLock::new(Vec::new());

                Ok(Some(Arc::new(Self::new(config, pty, buf))))
            }

            None => Ok(None),
        }
    }

    pub fn update_pty(&self, input: &WinitInputHelper) -> anyhow::Result<()> {
        let text = input
            .text()
            .into_iter()
            .map(|c| match c {
                TextChar::Char(c) => c,
                TextChar::Back => '\u{8}',
            })
            .collect();

        self.pty.write(text)?;

        Ok(())
    }

    pub fn spawn_worker(self: Arc<Self>, font: Arc<LoadedFont>) {
        thread::spawn(move || loop {
            match self.pty.read() {
                Ok(content) => {
                    for c in content.chars() {
                        if let Some(chr) = font.get_chr_by_id(c) {
                            self.buf.write().unwrap().push(chr);
                        }
                    }
                }

                Err(e) => match e.downcast_ref::<nix::errno::Errno>() {
                    Some(nix::errno::Errno::EBADF) => break,
                    _ => {}
                },
            }
        });
    }
}
