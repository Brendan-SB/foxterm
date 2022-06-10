mod loaded_font;
mod mesh;
mod renderer;
mod shaders;
mod terminal;
mod texture;

use renderer::Renderer;
use std::rc::Rc;
use terminal::Terminal;

fn main() {
    let terminal = match Terminal::init().unwrap() {
        Some(terminal) => Rc::new(terminal),
        None => return,
    };

    Renderer::init(None, terminal).unwrap();
}
