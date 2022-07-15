pub mod config;
pub mod drawable;
pub mod pty;

use crate::loaded_font::{chr::Chr, LoadedFont};
use cgmath::{Array, Vector2, Vector4, Zero};
use config::Config;
use crossbeam::channel::{self, Sender};
use drawable::Drawable;
use pty::Pty;
use std::{
    env,
    sync::{Arc, RwLock},
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

    pub fn spawn_reader(&self, font: Arc<LoadedFont>) -> Arc<RwLock<Performer>> {
        let pty = self.pty.clone();
        let screen = self.screen.clone();
        let performer = Arc::new(RwLock::new(Performer::default(
            font.clone(),
            screen.clone(),
        )));

        {
            let performer = performer.clone();

            thread::spawn(move || loop {
                match pty.read() {
                    Ok(buf) => {
                        let mut parser = Parser::new();

                        for u in buf {
                            performer.write().unwrap().advance_parser(&mut parser, u);
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

        performer
    }

    pub fn spawn_writer(&self) -> Sender<Vec<u8>> {
        let (sender, receiver) = channel::unbounded();
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

pub struct Performer {
    pub font: Arc<LoadedFont>,
    pub screen: Arc<RwLock<Vec<Drawable>>>,
    pub color: Vector4<f32>,
    pub pos: Vector2<f32>,
}

impl Performer {
    pub fn new(
        font: Arc<LoadedFont>,
        screen: Arc<RwLock<Vec<Drawable>>>,
        color: Vector4<f32>,
        pos: Vector2<f32>,
    ) -> Self {
        Self {
            font,
            screen,
            color,
            pos,
        }
    }

    pub fn default(font: Arc<LoadedFont>, screen: Arc<RwLock<Vec<Drawable>>>) -> Self {
        Self::new(
            font.clone(),
            screen,
            Vector4::zero(),
            Vector2::from_value(-1.0),
        )
    }

    fn add_char(&mut self, chr: Arc<Chr>) {
        let mut screen = self.screen.write().unwrap();

        self.pos.x += chr.bearing.x;

        let mut pos = self.pos;

        pos.y += chr.bearing.y;

        screen.push(Drawable::new(chr.clone(), pos));

        self.pos.x += chr.dimensions.x;

        update_pos(&mut self.pos, self.font.scale, &mut *screen)
    }

    fn advance_parser(&mut self, parser: &mut Parser, u: u8) {
        if u == 8 {
            let mut screen = self.screen.write().unwrap();
            let min_pos = {
                let mut min_pos = self.pos - Vector2::new(self.font.scale / 2.0, 0.0);

                update_pos(&mut min_pos, self.font.scale, &mut screen);

                min_pos
            };

            for i in 0..screen.len() {
                if screen[i].pos.y == min_pos.y && screen[i].pos.x > min_pos.x {
                    self.pos = screen[i].pos;

                    screen.remove(i);

                    break;
                }
            }
        } else if u == 20 {
            self.pos.x += self.font.scale;

            update_pos(
                &mut self.pos,
                self.font.scale,
                &mut *self.screen.write().unwrap(),
            )
        } else {
            parser.advance(self, u);
        }
    }
}

impl Perform for Performer {
    fn print(&mut self, c: char) {
        if let Some(chr) = self.font.get_chr_by_id(c as u8) {
            self.add_char(chr);
        }
    }

    fn csi_dispatch(
        &mut self,
        params: &Params,
        _intermediates: &[u8],
        _ignore: bool,
        action: char,
    ) {
        match action {
            'K' => match params.iter().next() {
                Some([0] | []) => {
                    self.pos.x = 1.0 + self.font.scale;
                }
                _ => {}
            },
            'C' => match params.iter().next() {
                Some([n]) => {
                    self.pos.x += self.font.scale * *n as f32;
                }
                Some([]) => {
                    self.pos.x += self.font.scale;
                }
                _ => {}
            },
            _ => {}
        }

        update_pos(
            &mut self.pos,
            self.font.scale,
            &mut *self.screen.write().unwrap(),
        )
    }
}

fn update_x(pos: &mut Vector2<f32>, scale: f32) {
    if pos.x > 1.0 - scale {
        *pos = Vector2::new(-2.0 + pos.x + scale, pos.y + scale);
    } else if pos.x < -1.0 {
        *pos = Vector2::new(2.0 + pos.x - scale, pos.y - scale);
    }
}

fn update_y(pos: &mut Vector2<f32>, scale: f32, screen: &mut Vec<Drawable>) {
    if pos.y > 1.0 - scale {
        pos.y = 2.0 - scale - pos.y;

        screen.retain_mut(|d| {
            d.pos.y -= scale;

            d.pos.y > -1.0
        });
    }
}

fn update_pos(pos: &mut Vector2<f32>, scale: f32, screen: &mut Vec<Drawable>) {
    update_x(pos, scale);
    update_y(pos, scale, screen);
}
