//! Remote SSH Bridge for MachTUI.
//! Provides a high-level wrapper for remote command execution and state polling.

use ssh2::Session;
use std::net::TcpStream;
use std::io::Read;

pub struct SshBridge {
    pub session: Session,
}

impl SshBridge {
    /// Connects to a remote host using SSH.
    pub fn connect(addr: &str, username: &str, password: &str) -> Result<Self, String> {
        let tcp = TcpStream::connect(addr).map_err(|e| e.to_string())?;
        let mut sess = Session::new().map_err(|e| e.to_string())?;
        sess.set_tcp_stream(tcp);
        sess.handshake().map_err(|e| e.to_string())?;
        sess.userauth_password(username, password).map_err(|e| e.to_string())?;
        
        if !sess.authenticated() {
            return Err("Authentication failed".into());
        }
        
        Ok(Self { session: sess })
    }

    /// Executes a command on the remote host and returns the output.
    pub fn execute(&self, command: &str) -> Result<String, String> {
        let mut channel = self.session.channel_session().map_err(|e| e.to_string())?;
        channel.exec(command).map_err(|e| e.to_string())?;
        let mut s = String::new();
        channel.read_to_string(&mut s).map_err(|e| e.to_string())?;
        channel.wait_close().map_err(|e| e.to_string())?;
        Ok(s)
    }
}
