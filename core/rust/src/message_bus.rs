use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub ts: DateTime<Utc>,
    pub topic: String,
    pub payload: Vec<u8>,
}

impl Message {
    pub fn new(topic: impl Into<String>, payload: impl Into<Vec<u8>>) -> Self {
        Self {
            id: Uuid::new_v4(),
            ts: Utc::now(),
            topic: topic.into(),
            payload: payload.into(),
        }
    }
}

#[derive(Clone)]
pub struct MessageBus {
    tx: broadcast::Sender<Arc<Message>>,
    log: Arc<RwLock<VecDeque<Arc<Message>>>>,
    capacity: usize,
}

impl MessageBus {
    pub fn new(event_log_capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(1024);
        Self {
            tx,
            log: Arc::new(RwLock::new(VecDeque::with_capacity(event_log_capacity))),
            capacity: event_log_capacity,
        }
    }

    pub fn publish(&self, msg: Message) {
        let msg = Arc::new(msg);
        {
            let mut log = self.log.write().expect("message bus log lock");
            if log.len() == self.capacity {
                log.pop_front();
            }
            log.push_back(msg.clone());
        }
        let _ = self.tx.send(msg);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Arc<Message>> {
        self.tx.subscribe()
    }

    pub fn snapshot_log(&self) -> Vec<Arc<Message>> {
        self.log
            .read()
            .expect("message bus log lock")
            .iter()
            .cloned()
            .collect()
    }
}
