use async_trait::async_trait;
use tokio::time::{Duration, Instant};
use tracing::info;

use crate::{
    interfaces::{Component, ComponentContext},
    message_bus::Message,
};

pub struct HeartbeatComponent {
    interval: Duration,
}

impl HeartbeatComponent {
    pub fn new() -> Self {
        Self {
            interval: Duration::from_secs(1),
        }
    }
}

#[async_trait]
impl Component for HeartbeatComponent {
    fn name(&self) -> &'static str {
        "heartbeat"
    }

    async fn start(
        &self,
        ctx: ComponentContext,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let bus = ctx.bus;
        let interval = self.interval;

        tokio::spawn(async move {
            let mut next = Instant::now() + interval;
            loop {
                tokio::time::sleep_until(next).await;
                next += interval;

                bus.publish(Message::new("core.heartbeat", b"tick".to_vec()));
            }
        });

        Ok(())
    }
}

pub struct EventLoggerComponent;

impl EventLoggerComponent {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Component for EventLoggerComponent {
    fn name(&self) -> &'static str {
        "event_logger"
    }

    async fn start(
        &self,
        ctx: ComponentContext,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut rx = ctx.bus.subscribe();

        tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                let payload = String::from_utf8_lossy(&msg.payload);
                info!(topic = %msg.topic, payload = %payload, "event");
            }
        });

        Ok(())
    }
}
