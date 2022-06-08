use crate::{config::Config, pty::Pty};
use std::{cell::RefCell, rc::Rc};

pub struct Terminal {
    pub pty: Pty,
    pub config: Config,
}

impl Terminal {
    pub fn new(pty: Pty, config: Config) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { pty, config }))
    }
}
