# PostgreSQL latency test

## What is it?

This small utility shows the replication latency between two PG instances. It basically increments a counter on the primary and measures the time it takes for that change to be propagated to the secondary. It can optionally publish every data point to a Google Cloud pub/sub topic of your choosing. This way, with the automatic BigQuery subscription it's possibile to store and analyze tons of datapoints with ease.

## Usage

Compile the tool using Rust (ie `cargo install --path .`) and launch it from the command line. As for PostgreSQL, create a table and insert a single row with this simple script: 

```sql
CREATE TABLE tbl(id INT, value INT);                                                              

INSERT INTO tbl(id, value) VALUES(1, 900);
```

Required parameters for the tool are the two connection strings (in the format specificed here: [https://docs.rs/tokio-postgres/latest/tokio_postgres/config/struct.Config.html](https://docs.rs/tokio-postgres/latest/tokio_postgres/config/struct.Config.html)). You can also optionally change the sleep time between tries and the pub/sub topic to write to.

For example this command:

```bash
pgdelaytest --primary-connection-string "host=primary user=test password=password" --secondary-connection-string "host=secondary user=test password=password" publish --pub-sub-topic pglatency
```

Tests the latency between `primary` and `secondary`, publishing the results both to stdout and to the GCP topic `pglatency` for streaming to BigQuery.

Note that, in order to publish to pub/sub, a valid GCP identity must be available and proper permissions must be granted.

## Usage (Docker)

You can either build the container with `docker build . -t pgdelaytest:latest` or pull it from Docker.io. Then execute it passing env variables. For example:

```bash
docker run -e PRIMARY_CONNECTION_STRING="host=host user=test password=password" -e SECONDARY_CONNECTION_STRING="host=secondary user=test password=password" -e PUB_SUB_TOPIC=topic -e GOOGLE_APPLICATION_CREDENTIALS=/service_account_pvk.json -v /service_account_pvk.json:/service_account_pvk.json pgdelaytest:latest
```

*Note*: this example uses a service account key file, it is not necessary if you don't want to publish to pub/sub or you have default credentials at hand.

## Methodology

The tool updates a row on the primary and right away tries to get the same row from the secondary. If the value matches, the reported latency is zero. If not, the tool keeps querying the same row until the value matches and then reports the time taken as *replication latency*. 

This means two things:

1. Zero latency does not mean zero microseconds: it means the latency is so low that the tool is unable to determine it.
2. The latency measured, if bigger than zero, incorporates an error the depends on how fast the tool can query the secondary instance. It might be interpreted as an upper bound.

The tool is able to calculate milliseconds (or even microseconds) but given the constraints above I think it's best to give rough estimates in seconds. If you don't agree, please open an issue and I'll add the option.

## Pub/sub

The tool optionally publishes the event message to a pub/sub topic in GCP. This can be used to have the automatic BigQuery subscription stream data to BigQuery. You can find the schema of the message in the [schema.proto](extra/schema.proto) file. The BigQuery table definition is in the [create_table.sql](extra/create_table.sql) file.

For example, this query shows the latest entries:

```sql
SELECT TIMESTAMP_MILLIS(timestamp) AS event_time, latency_ms 
FROM `mind-lab-wash.someds.frompubsublatency` 
ORDER BY timestamp DESC
LIMIT 1000;
```

This can be useful to view the data graphically via Looker Studio or Pro.
