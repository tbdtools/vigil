use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{broadcast, mpsc};
use tracing::{debug, error, info};

/// Represents a system event captured by Vigil
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Unique identifier for the event
    pub id: String,

    /// Timestamp when the event occurred (in nanoseconds since UNIX epoch)
    pub timestamp: u64,

    /// Type of event (e.g., "process_exec", "file_open", "network_connect")
    pub event_type: String,

    /// Process that generated the event
    pub process: ProcessInfo,

    /// Additional event-specific data
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: i32,
    pub ppid: i32,
    pub uid: u32,
    pub gid: u32,
    pub comm: String,
    pub exe: String,
}

#[derive(Error, Debug)]
pub enum EventError {
    #[error("Failed to collect event: {0}")]
    CollectionError(String),

    #[error("Failed to process event: {0}")]
    ProcessingError(String),

    #[error("Failed to store event: {0}")]
    StorageError(String),
}

#[async_trait::async_trait]
pub trait EventProcessor: Send + Sync {
    /// Process a single event and optionally transform it
    async fn process(&self, event: &Event) -> Result<Option<Event>, EventError>;

    /// Name of the processor for identification
    fn name(&self) -> &str;
}

#[async_trait::async_trait]
pub trait EventCollector: Send + Sync {
    /// Start collecting events
    async fn start(&mut self) -> Result<(), EventError>;

    /// Stop collecting events
    async fn stop(&mut self) -> Result<(), EventError>;

    /// Name of the collector for identification
    fn name(&self) -> &str;
}

#[async_trait::async_trait]
pub trait EventStorage: Send + Sync {
    /// Store a single event
    async fn store(&self, event: &Event) -> Result<(), EventError>;

    /// Query events based on criteria
    async fn query(&self, query: &EventQuery) -> Result<Vec<Event>, EventError>;
}

#[derive(Debug, Clone)]
pub struct EventQuery {
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub event_types: Option<Vec<String>>,
    pub process_filter: Option<ProcessFilter>,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct ProcessFilter {
    pub pid: Option<i32>,
    pub comm: Option<String>,
    pub uid: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct EventConfig {
    pub buffer_size: usize,
    pub batch_size: usize,
    pub processor_parallelism: usize,
}

impl Default for EventConfig {
    fn default() -> Self {
        Self {
            buffer_size: 10000,
            batch_size: 100,
            processor_parallelism: 4,
        }
    }
}

pub struct EventPipeline {
    config: EventConfig,
    collectors: Vec<Box<dyn EventCollector>>,
    processors: Vec<Arc<dyn EventProcessor>>,
    storage: Arc<dyn EventStorage>,
    event_tx: broadcast::Sender<Event>,
    shutdown_tx: mpsc::Sender<()>,
}

impl EventPipeline {
    pub fn new(
        config: EventConfig,
        collectors: Vec<Box<dyn EventCollector>>,
        processors: Vec<Arc<dyn EventProcessor>>,
        storage: Arc<dyn EventStorage>,
    ) -> Self {
        let (event_tx, _) = broadcast::channel(config.buffer_size);
        let (shutdown_tx, _) = mpsc::channel(1);

        Self {
            config,
            collectors,
            processors,
            storage,
            event_tx,
            shutdown_tx,
        }
    }

    pub async fn start(&mut self) -> Result<(), EventError> {
        info!("Starting event pipeline");

        for collector in &mut self.collectors {
            collector.start().await.map_err(|e| {
                error!("Failed to start collector {}: {}", collector.name(), e);
                e
            })?;
        }

        self.spawn_workers().await;

        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), EventError> {
        info!("Stopping event pipeline");

        if let Err(e) = self.shutdown_tx.send(()).await {
            error!("Failed to send shutdown signal: {}", e);
        }

        for collector in &mut self.collectors {
            collector.stop().await.map_err(|e| {
                error!("Failed to stop collector {}: {}", collector.name(), e);
                e
            })?;
        }

        Ok(())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.event_tx.subscribe()
    }

    async fn spawn_workers(&self) {
        let event_tx = self.event_tx.clone();
        let processors = self.processors.clone();
        let storage = self.storage.clone();
        let batch_size = self.config.batch_size;

        for _ in 0..self.config.processor_parallelism {
            let event_rx = self.event_tx.subscribe();
            let processors = processors.clone();
            let storage = storage.clone();
            let event_tx = event_tx.clone();

            tokio::spawn(async move {
                Self::process_events(event_rx, processors, storage, event_tx, batch_size).await;
            });
        }
    }

    async fn process_events(
        mut event_rx: broadcast::Receiver<Event>,
        processors: Vec<Arc<dyn EventProcessor>>,
        storage: Arc<dyn EventStorage>,
        event_tx: broadcast::Sender<Event>,
        batch_size: usize,
    ) {
        let mut batch = Vec::with_capacity(batch_size);

        while let Ok(event) = event_rx.recv().await {
            let mut current_event = event.clone();

            // Apply all processors in sequence
            for processor in &processors {
                match processor.process(&current_event).await {
                    Ok(Some(processed_event)) => {
                        current_event = processed_event;
                    }
                    Ok(None) => {
                        debug!("Event filtered by processor {}", processor.name());
                        continue;
                    }
                    Err(e) => {
                        error!("Error processing event: {}", e);
                        continue;
                    }
                }
            }

            // Add to batch
            batch.push(current_event.clone());

            // Forward processed event
            if let Err(e) = event_tx.send(current_event) {
                error!("Failed to forward processed event: {}", e);
            }

            // Store batch if full
            if batch.len() >= batch_size {
                Self::store_batch(&storage, &batch).await;
                batch.clear();
            }
        }

        // Store any remaining events
        if !batch.is_empty() {
            Self::store_batch(&storage, &batch).await;
        }
    }

    async fn store_batch(storage: &Arc<dyn EventStorage>, batch: &[Event]) {
        for event in batch {
            if let Err(e) = storage.store(event).await {
                error!("Failed to store event: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[tokio::test]
    async fn test_event_pipeline() {
        // Create a test event
        let event = Event {
            id: "test-1".into(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
            event_type: "test".into(),
            process: ProcessInfo {
                pid: 1234,
                ppid: 1,
                uid: 1000,
                gid: 1000,
                comm: "test".into(),
                exe: "/bin/test".into(),
            },
            data: serde_json::json!({"test": true}),
        };

        assert!(event.timestamp > 0);
    }
}
