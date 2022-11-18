use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// publish events to Google Cloud Pub/Sub
    Publish {
        /// Pub/sub topic to publish to
        #[arg(long)]
        pub_sub_topic: String,

        /// Service account key path
        #[arg(long)]
        service_account_key_path: String,
    },
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Options {
    /// Primary instance (RW)
    #[arg(short, long)]
    pub primary_connection_string: String,

    /// Secondary instance (RO)
    #[arg(short, long)]
    pub secondary_connection_string: String,

    /// Sleep time in ms after each evaluation
    #[arg(long, default_value_t = 1000)]
    pub sleep_ms: u64,

    #[command(subcommand)]
    pub command: Option<Commands>,
}
