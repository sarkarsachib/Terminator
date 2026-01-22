pub mod builtin;

use async_trait::async_trait;

use crate::message_bus::Message;

#[derive(Debug, Clone)]
pub struct ToolCall {
    pub name: String,
    pub input: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ToolResult {
    pub output: Vec<u8>,
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &'static str;

    async fn invoke(
        &self,
        call: ToolCall,
    ) -> Result<ToolResult, Box<dyn std::error::Error + Send + Sync>>;
}

pub fn tool_message(topic: &str, payload: impl Into<Vec<u8>>) -> Message {
    Message::new(topic, payload)
}
