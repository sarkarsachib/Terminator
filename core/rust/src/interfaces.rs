use async_trait::async_trait;

use crate::{config::TerminatorConfig, message_bus::MessageBus};

#[derive(Clone)]
pub struct ComponentContext {
    pub bus: MessageBus,
    pub config: TerminatorConfig,
}

impl ComponentContext {
    pub fn new(bus: MessageBus, config: TerminatorConfig) -> Self {
        Self { bus, config }
    }
}

#[async_trait]
pub trait Component: Send + Sync {
    fn name(&self) -> &'static str;

    async fn start(
        &self,
        ctx: ComponentContext,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
