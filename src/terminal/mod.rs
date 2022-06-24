pub mod config;
pub mod drawable;
pub mod pty;

use crate::loaded_font::LoadedFont;
use cgmath::{Vector2, Vector4, Zero};
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
use vte::{Params, Parser, Perform};
use winit::event::VirtualKeyCode;
use winit_input_helper::{TextChar, WinitInputHelper};

pub struct Terminal {
    pub config: Config,
    pub pty: Arc<Pty>,
    pub screen: Arc<RwLock<Vec<Drawable>>>,
}

impl Terminal {
    pub fn new(config: Config, pty: Arc<Pty>, screen: Arc<RwLock<Vec<Drawable>>>) -> Self {
        Self {
            config,
            pty,
            screen,
        }
    }

    pub fn init() -> anyhow::Result<Option<Self>> {
        match Pty::spawn(env::var("SHELL").unwrap().to_owned())? {
            Some(pty) => {
                let config = Config::new(None, [0.0; 4], "test.ttf".to_owned(), [1.0; 4], 40.0);

                Ok(Some(Self::new(
                    config,
                    pty,
                    Arc::new(RwLock::new(Vec::new())),
                )))
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

    pub fn spawn_reader(&self, font: Arc<LoadedFont>) {
        let pty = self.pty.clone();
        let screen = self.screen.clone();

        thread::spawn(move || loop {
            match pty.read() {
                Ok(buf) => {
                    let mut screen = screen.write().unwrap();
                    let mut performer = Performer::default(font.clone(), &mut screen);
                    let mut parser = Parser::new();

                    for u in buf {
                        parser.advance(&mut performer, u);
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

    pub fn spawn_writer(&self) -> Sender<Vec<u8>> {
        let (sender, receiver) = mpsc::channel();
        let pty = self.pty.clone();

        thread::spawn(move || loop {
            match receiver.recv() {
                Ok(content) => {
                    if let Err(e) = pty.write(&content) {
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

struct Performer<'a> {
    font: Arc<LoadedFont>,
    screen: &'a mut Vec<Drawable>,
    color: Vector4<f32>,
    advance: u16,
}

impl<'a> Performer<'a> {
    fn new(
        font: Arc<LoadedFont>,
        screen: &'a mut Vec<Drawable>,
        color: Vector4<f32>,
        advance: u16,
    ) -> Self {
        Self {
            font,
            screen,
            color,
            advance,
        }
    }

    fn default(font: Arc<LoadedFont>, screen: &'a mut Vec<Drawable>) -> Self {
        Self::new(font, screen, Vector4::zero(), 0)
    }
}

impl<'a> Perform for Performer<'a> {
    fn print(&mut self, c: char) {
        if let Some(chr) = self.font.get_chr_by_id(c as u8) {
            let drawable = match self.screen.last() {
                Some(drawable) => {
                    let pos = {
                        let x = drawable.pos.x + drawable.chr.dimensions.x + chr.bearing.x;

                        if x >= 1.0 {
                            Vector2::new(-1.0, drawable.pos.y + self.font.scale)
                        } else {
                            Vector2::new(x, drawable.pos.y)
                        }
                    };

                    Drawable::new(chr, pos)
                }
                None => Drawable::new(chr, Vector2::new(1.0, -1.0 - self.font.scale)),
            };

            self.screen.push(drawable);
        }
    }

    fn csi_dispatch(
        &mut self,
        params: &Params,
        _intermediates: &[u8],
        _ignore: bool,
        action: char,
    ) {
        if action == 'C' {
            if let Some(param) = params.iter().nth(0) {
                self.advance = param[0] as u16;
            }
        }
    }
}
