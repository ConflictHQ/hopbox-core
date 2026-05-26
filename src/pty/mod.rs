// PTY management primitives.
// Thin wrapper around portable-pty to provide hopbox-specific types.

pub use portable_pty::{CommandBuilder, PtySize};
use crate::Result;
use crate::error::Error;

pub struct PtyHandle {
    pub master: Box<dyn portable_pty::MasterPty + Send>,
    pub child: Box<dyn portable_pty::Child + Send + Sync>,
}

pub fn spawn_shell(cols: u16, rows: u16) -> Result<PtyHandle> {
    let pty_system = portable_pty::native_pty_system();
    let size = PtySize { rows, cols, pixel_width: 0, pixel_height: 0 };
    let pair = pty_system.openpty(size).map_err(|e| Error::Pty(e.to_string()))?;

    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());
    let cmd = CommandBuilder::new(shell);
    let child = pair.slave.spawn_command(cmd).map_err(|e| Error::Pty(e.to_string()))?;

    Ok(PtyHandle { master: pair.master, child })
}
