mod event;
mod options;

use anyhow::{Context, Error};
use clap::Parser;
use cloud_pubsub::Client as PubSubClient;
use event::Event;
use options::{Commands, Options};
use tokio_postgres::*;

const ID: i32 = 1;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let options = Options::parse();
    // println!("Using options {:?}", options);

    let topic = if let Some(Commands::Publish {
        pub_sub_topic,
        service_account_key_path,
    }) = options.command
    {
        let pubsub = PubSubClient::new(service_account_key_path).await?;
        println!("pubsub.project == {:?}", pubsub.project());
        let topic = pubsub.topic(pub_sub_topic.clone());
        Some(topic)
    } else {
        None
    };
    println!("Openinig connection to primary...");
    let (pri_client, pri_connection) =
        tokio_postgres::connect(&options.primary_connection_string, NoTls).await?;
    println!("Openinig connection to secondary...");
    let (sec_client, sec_connection) =
        tokio_postgres::connect(&options.secondary_connection_string, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = pri_connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    println!("Connection to primary openened");

    tokio::spawn(async move {
        if let Err(e) = sec_connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    println!("Connection to secondary openened");

    let mut val_to_set = 0;
    loop {
        val_to_set += 1;
        let _rows_updated = update(&pri_client, val_to_set).await?;

        let start = std::time::Instant::now();
        let mut looped = false;

        loop {
            let val_retrieved = get_value(&sec_client).await?;

            if val_to_set == val_retrieved {
                break;
            } else {
                looped = true;
            }
        }

        let ms_elapsed = if looped {
            start.elapsed().as_millis() as u64
        } else {
            0
        };
        let event: Event = ms_elapsed.into();
        if let Some(ref topic) = topic {
            topic.publish(&event).await?;
        }
        println!("delay {:?} published", event);

        if ms_elapsed < options.sleep_ms {
            let duration_to_wait = std::time::Duration::from_millis(options.sleep_ms - ms_elapsed);
            tokio::time::sleep(duration_to_wait).await;
        };
    }

    #[allow(unreachable_code)]
    Ok(())
}

async fn update(client: &Client, val: i32) -> Result<u64, Error> {
    client
        .execute("UPDATE tbl SET value = $1 WHERE id = $2", &[&val, &ID])
        .await
        .with_context(|| format!("failed to update value to {}", val))
}

async fn get_value(client: &Client) -> Result<i32, Error> {
    let row = client
        .query_one("SELECT value FROM tbl WHERE id = $1", &[&ID])
        .await
        .with_context(|| "failed to retrieve value")?;

    row.try_get(0)
        .map_err(|e| anyhow::Error::msg(e.to_string()))
}
