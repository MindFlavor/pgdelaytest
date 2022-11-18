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
