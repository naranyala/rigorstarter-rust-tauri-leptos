use tokio::sync::broadcast;

#[derive(Clone, Debug)]
pub enum AppEvent {
    Error(String),
    Info(String),
    Success(String),
}

pub struct EventBus {
    tx: broadcast::Sender<AppEvent>,
}

impl EventBus {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        Self { tx }
    }

    /// Publish an event to all subscribers
    pub fn publish(&self, event: AppEvent) {
        let _ = self.tx.send(event);
    }

    /// Subscribe to the event stream
    pub fn subscribe(&self) -> broadcast::Receiver<AppEvent> {
        self.tx.subscribe()
    }
}
