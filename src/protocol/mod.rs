use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    Attach { session_id: Option<String> },
    Detach,
    Input { data: Vec<u8> },
    Resize { cols: u16, rows: u16 },
    AiQuery { query: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    SessionReady { session_id: String },
    Output { data: Vec<u8> },
    AiResponse { content: String, suggested_command: Option<String> },
    Error { message: String },
}
