use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use std::time::SystemTime;

#[derive(Debug, Copy, Clone, serde::Serialize)]
pub struct Event {
    timestamp: u128,
    latency_ms: u64,
}

impl Event {}

impl From<u64> for Event {
    fn from(value: u64) -> Self {
        Self {
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            latency_ms: value,
        }
    }
}

impl From<Event> for PubsubMessage {
    fn from(value: Event) -> Self {
        PubsubMessage {
            data: serde_json::to_string(&value).unwrap().as_bytes().to_vec(),
            ..Default::default()
        }
    }
}
