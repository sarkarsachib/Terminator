pub mod config;
pub mod interfaces;
pub mod message_bus;
pub mod output;
pub mod tools;

use crate::{
    config::TerminatorConfig,
    interfaces::{Component, ComponentContext},
    message_bus::MessageBus,
};

pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = TerminatorConfig::load_from_repo_defaults().unwrap_or_default();

    let bus = MessageBus::new(config.core.event_log_capacity);

    let mut orchestrator = Orchestrator::new(config, bus);
    orchestrator.register_component(crate::tools::builtin::HeartbeatComponent::new());
    orchestrator.register_component(crate::tools::builtin::EventLoggerComponent::new());

    orchestrator.start().await?;

    tokio::signal::ctrl_c().await?;
    Ok(())
}

pub struct Orchestrator {
    config: TerminatorConfig,
    bus: MessageBus,
    components: Vec<Box<dyn Component>>,
}

impl Orchestrator {
    pub fn new(config: TerminatorConfig, bus: MessageBus) -> Self {
        Self {
            config,
            bus,
            components: Vec::new(),
        }
    }

    pub fn register_component<C: Component>(&mut self, component: C) {
        self.components.push(Box::new(component));
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = ComponentContext::new(self.bus.clone(), self.config.clone());
        for component in &self.components {
            component.start(ctx.clone()).await?;
        }
        Ok(())
    }
}
