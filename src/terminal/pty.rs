use nix::{
    pty,
    unistd::{self, ForkResult},
};
use std::{os::unix::io::RawFd, process::Command, sync::Arc};

pub const BUFFER_SIZE: usize = 65536;

pub struct Pty {
    pub fd: RawFd,
}

impl Pty {
    pub fn new(fd: RawFd) -> Self {
        Self { fd }
    }

    pub fn spawn(shell_path: String) -> anyhow::Result<Option<Arc<Self>>> {
        let fork_pty = unsafe { pty::forkpty(None, None)? };

        match fork_pty.fork_result {
            ForkResult::Parent { .. } => Ok(Some(Arc::new(Self::new(fork_pty.master)))),
            ForkResult::Child => {
                let _ = Command::new(&shell_path).status()?;

                Ok(None)
            }
        }
    }

    pub fn read(&self) -> anyhow::Result<Vec<u8>> {
        let mut buffer = vec![0; BUFFER_SIZE];

        unistd::read(self.fd, &mut buffer)?;

        buffer.retain(|c| *c != 0);

        Ok(buffer)
    }

    pub fn write(&self, buffer: &[u8]) -> anyhow::Result<()> {
        unistd::write(self.fd, buffer)?;

        Ok(())
    }

    pub fn close(&self) -> anyhow::Result<()> {
        unistd::close(self.fd)?;

        Ok(())
    }
}

impl Drop for Pty {
    fn drop(&mut self) {
        let _ = self.close();
    }
}
