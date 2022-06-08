mod config;
mod mesh;
mod pty;
mod renderer;
mod shaders;
mod terminal;
mod texture;

use config::Config;
use pty::Pty;
use renderer::Renderer;
use std::env;
use terminal::Terminal;

fn main() {
    let pty = if let Some(pty) = Pty::spawn(env::var("SHELL").unwrap().to_owned()).unwrap() {
        pty
    } else {
        return;
    };
    let config = Config::new([0.0; 4], [1.0; 4]);
    let terminal = Terminal::new(pty, config);
    
    std::thread::sleep(std::time::Duration::from_millis(500));

    println!("{}", terminal.borrow_mut().pty.read().unwrap());

    Renderer::init(None, terminal.clone()).unwrap();
}
