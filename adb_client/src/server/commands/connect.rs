use crate::{ADBServer, Result, RustADBError, models::AdbServerCommand};
use std::net::SocketAddrV4;

impl ADBServer {
    /// Connect device over tcp with address and port
    pub fn connect_device(&mut self, address: SocketAddrV4) -> Result<()> {
        let response = self
            .connect()?
            .proxy_connection(AdbServerCommand::Connect(address), true)?;

        match String::from_utf8(response) {
            Ok(s) if s.starts_with("connected to") => Ok(()),
            Ok(s) => Err(RustADBError::ADBRequestFailed(s)),
            Err(e) => Err(e.into()),
        }
    }
}
