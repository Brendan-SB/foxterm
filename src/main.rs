mod loaded_font;
mod mesh;
mod renderer;
mod shaders;
mod terminal;
mod texture;

use renderer::Renderer;
use terminal::Terminal;

pub const APP_NAME: &str = "foxterm";
pub const SCALE: f32 = 1.0 / 1000.0;

fn main() {
    let terminal = match Terminal::init().unwrap() {
        Some(terminal) => terminal,
        None => return,
    };

    Renderer::init(None, terminal).unwrap();
}
