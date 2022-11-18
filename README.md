# PostgreSQL latency test

## What is it?

This small utility shows the replication latency between two PG instances. It basically increments a counter on the primary and measures the time it takes for that change to be propagated to the secondary. It can optionally publish every data point to a Google Cloud pub/sub topic of your choosing. This way, with the automatic BigQuery subscription it's possibile to store and analyze tons of datapoints with ease.

## Usage

Compile the tool using Rust (ie `cargo install --path .`) and launch it from the command line.

Required parameters are the two connection strings (in the format specificed here: [https://docs.rs/tokio-postgres/latest/tokio_postgres/config/struct.Config.html](https://docs.rs/tokio-postgres/latest/tokio_postgres/config/struct.Config.html)). You can also optionally change the sleep time between tries and the pub/sub topic to write to. If you specify the pub/sub topic you need to specify a service account key file path that has enough privileges to write to the topic.


