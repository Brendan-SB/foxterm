mod loaded_font;
mod mesh;
mod renderer;
mod shaders;
mod terminal;
mod texture;

use renderer::Renderer;
use terminal::Terminal;

fn main() {
    let terminal = match Terminal::init().unwrap() {
        Some(terminal) => terminal,
        None => return,
    };

    std::thread::sleep(std::time::Duration::from_millis(500));

    println!("{}", terminal.borrow_mut().pty.read().unwrap());

    Renderer::init(None, terminal.clone()).unwrap();
}
