//! AI JSON-RPC Server for MachTUI.
//! Allows external agents to interact with the TUI via TCP.

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::talon::{Model, Program};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct OracleRequest {
    method: String, // "get_ui" or "dispatch"
    params: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
struct OracleResponse {
    result: Option<serde_json::Value>,
    error: Option<String>,
}

pub async fn start_ai_server<M: Model>(program: &Program<M>, port: u16) -> tokio::io::Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
    println!("MachTUI Oracle Server listening on port {}", port);

    loop {
        let (mut socket, _) = listener.accept().await?;
        let ui_json = program.oracle_json();
        
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            if let Ok(n) = socket.read(&mut buf).await {
                if n == 0 { return; }
                
                // Simple responder: return current UI state for any request
                let response = OracleResponse {
                    result: Some(serde_json::from_str(&ui_json).unwrap_or(serde_json::Value::Null)),
                    error: None,
                };
                
                let resp_bytes = serde_json::to_vec(&response).unwrap();
                let _ = socket.write_all(&resp_bytes).await;
            }
        });
    }
}
