use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone)]
pub struct AiContext {
    pub messages: Vec<Message>,
    pub terminal_buffer: Option<String>,
    pub system_prompt: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AiResponse {
    pub content: String,
    pub suggested_command: Option<String>,
}

#[async_trait]
pub trait AiProvider: Send + Sync {
    async fn complete(&self, ctx: &AiContext) -> Result<AiResponse>;
    fn name(&self) -> &'static str;
}
