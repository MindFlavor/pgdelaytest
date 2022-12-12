use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Options {
    /// Primary instance (RW)
    #[arg(short, long, env = "PRIMARY_CONNECTION_STRING")]
    pub primary_connection_string: String,

    /// Secondary instance (RO)
    #[arg(short, long, env = "SECONDARY_CONNECTION_STRING")]
    pub secondary_connection_string: String,

    /// Sleep time in ms after each evaluation
    #[arg(long, default_value_t = 1000, env = "SLEEP_MS")]
    pub sleep_ms: u64,

    /// Pub/sub topic to publish to
    #[arg(long, env = "PUB_SUB_TOPIC")]
    pub pub_sub_topic: Option<String>,
}
