pub mod config;
pub mod drawable;
pub mod pty;

use crate::loaded_font::LoadedFont;
use cgmath::Vector2;
use config::Config;
use drawable::Drawable;
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
    pub screen: RwLock<Vec<Drawable>>,
}

impl Terminal {
    pub fn new(config: Config, pty: Pty, screen: RwLock<Vec<Drawable>>) -> Self {
        Self {
            config,
            pty,
            screen,
        }
    }

    pub fn init() -> anyhow::Result<Option<Arc<Self>>> {
        match Pty::spawn(env::var("SHELL").unwrap().to_owned())? {
            Some(pty) => {
                let config = Config::new(None, [0.0; 4], "test.ttf".to_owned(), [1.0; 4], 40.0);

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
        } else if input.key_pressed(VirtualKeyCode::Tab) {
            text.push('\t' as u8);
        } else if input.key_pressed(VirtualKeyCode::LControl)
            || input.key_pressed(VirtualKeyCode::RControl)
        {
            text.push('^' as u8);
        }

        sender.send(text)?;

        Ok(())
    }

    pub fn spawn_reader(self: Arc<Self>, font: Arc<LoadedFont>) {
        thread::spawn(move || loop {
            match self.pty.read() {
                Ok(buf) => {
                    let mut screen = self.screen.write().unwrap();

                    for u in buf {
                        if let Some(chr) = font.get_chr_by_id(u) {
                            let drawable = match screen.last() {
                                Some(drawable) => {
                                    let pos = {
                                        let x = drawable.pos.x
                                            + drawable.chr.dimensions.x
                                            + chr.bearing.x;

                                        if x >= 1.0 {
                                            Vector2::new(-1.0, drawable.pos.y + font.scale)
                                        } else {
                                            Vector2::new(x, drawable.pos.y)
                                        }
                                    };

                                    Drawable::new(chr, pos)
                                }
                                None => Drawable::new(chr, Vector2::new(1.0, -1.0 - font.scale)),
                            };

                            screen.push(drawable);
                        }
                    }
                }
                Err(e) => match e.downcast_ref::<nix::errno::Errno>() {
                    Some(nix::errno::Errno::EBADF) => break,
                    _ => {
                        println!("Error on read: {:?}", e);
                    }
                },
            }
        });
    }

    pub fn spawn_writer(self: Arc<Self>) -> Sender<Vec<u8>> {
        let (sender, receiver) = mpsc::channel();

        thread::spawn(move || loop {
            match receiver.recv() {
                Ok(content) => {
                    if let Err(e) = self.pty.write(&content) {
                        match e.downcast_ref::<nix::errno::Errno>() {
                            Some(nix::errno::Errno::EBADF) => break,
                            _ => {
                                println!("Error on write: {:?}", e);
                            }
                        }
                    }
                }
                Err(_) => break,
            }
        });

        sender
    }
}
