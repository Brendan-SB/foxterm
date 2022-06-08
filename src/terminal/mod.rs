pub mod config;
pub mod pty;

use config::Config;
use pty::Pty;
use std::{cell::RefCell, env, rc::Rc};

pub struct Terminal {
    pub pty: Pty,
    pub config: Config,
}

impl Terminal {
    pub fn new(pty: Pty, config: Config) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { pty, config }))
    }

    pub fn init() -> anyhow::Result<Option<Rc<RefCell<Self>>>> {
        match Pty::spawn(env::var("SHELL").unwrap().to_owned())? {
            Some(pty) => {
                let config = Config::new([0.0; 4], [1.0; 4]);

                Ok(Some(Self::new(pty, config)))
            }

            None => Ok(None),
        }
    }
}
