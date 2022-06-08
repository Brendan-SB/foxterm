use nix::{pty, unistd::ForkResult};
use std::{fs::File, io::Read, os::unix::io::FromRawFd, process::Command};

pub const BUFFER_SIZE: usize = 65536;

pub struct Pty {
    pub file: File,
}

impl Pty {
    pub fn new(file: File) -> Self {
        Self { file }
    }

    pub fn spawn(shell_path: String) -> anyhow::Result<Option<Self>> {
        let fork_pty = unsafe { pty::forkpty(None, None)? };

        match fork_pty.fork_result {
            ForkResult::Parent { .. } => Ok(Some(Self::new(unsafe {
                File::from_raw_fd(fork_pty.master)
            }))),

            ForkResult::Child => {
                let _ = Command::new(&shell_path).status()?;

                Ok(None)
            }
        }
    }

    pub fn read(&mut self) -> anyhow::Result<String> {
        let mut buffer = vec![0; BUFFER_SIZE];

        self.file.read(&mut buffer)?;

        buffer.retain(|c| *c != 0);

        Ok(String::from_utf8(buffer)?)
    }
}
