pub mod config;
pub mod pty;

use crate::loaded_font::{chr::Chr, LoadedFont};
use config::Config;
use pty::Pty;
use std::{
    env,
    sync::{
        mpsc::{self, Sender},
        Arc, RwLock,
    },
    thread,
};
use winit::event::VirtualKeyCode;
use winit_input_helper::{TextChar, WinitInputHelper};

pub struct Terminal {
    pub config: Config,
    pub pty: Pty,
    pub content: RwLock<Vec<Content>>,
}

impl Terminal {
    pub fn new(config: Config, pty: Pty, content: RwLock<Vec<Content>>) -> Self {
        Self {
            config,
            pty,
            content,
        }
    }

    pub fn init() -> anyhow::Result<Option<Arc<Self>>> {
        match Pty::spawn(env::var("SHELL").unwrap().to_owned())? {
            Some(pty) => {
                let config = Config::new(Some(0), [0.0; 4], "test.ttf".to_owned(), [1.0; 4], 40.0);
                let pty = pty;

                Ok(Some(Arc::new(Self::new(
                    config,
                    pty,
                    RwLock::new(Vec::new()),
                ))))
            }

            None => Ok(None),
        }
    }

    pub fn update_pty(
        &self,
        sender: &Sender<Vec<u8>>,
        input: &WinitInputHelper,
    ) -> anyhow::Result<()> {
        let mut text = input
            .text()
            .into_iter()
            .map(|c| match c {
                TextChar::Char(c) => c as u8,
                TextChar::Back => '\u{8}' as u8,
            })
            .collect::<Vec<_>>();

        if input.key_pressed(VirtualKeyCode::Return) {
            text.push('\r' as u8);
        }

        sender.send(text)?;

        Ok(())
    }

    pub fn spawn_reader(self: Arc<Self>, font: Arc<LoadedFont>) {
        thread::spawn(move || loop {
            match self.pty.read().and_then(|c| Ok(String::from_utf8(c)?)) {
                Ok(content) => {
                    let mut content = content
                        .chars()
                        .map(|c| match font.get_chr_by_id(c) {
                            Some(chr) => Content::Chr(chr.clone()),
                            None => Content::Raw(c as u8),
                        })
                        .collect();

                    self.content.write().unwrap().append(&mut content);
                }

                Err(e) => match e.downcast_ref::<nix::errno::Errno>() {
                    Some(nix::errno::Errno::EBADF) => break,
                    e => {
                        println!("Error on read: {:?}", e);
                    }
                },
            }
        });
    }

    pub fn spawn_writer(self: Arc<Self>) -> Sender<Vec<u8>> {
        let (sender, receiver) = mpsc::channel();

        thread::spawn(move || loop {
            if let Ok(content) = receiver.recv() {
                if let Err(e) = self.pty.write(&content) {
                    println!("Error on write: {:?}", e);
                }
            }
        });

        sender
    }
}

pub enum Content {
    Chr(Arc<Chr>),
    Raw(u8),
}
