use std::time::Duration;

use futures::StreamExt;
use rdkafka::{
    ClientConfig, Message,
    consumer::{BaseConsumer, CommitMode, Consumer, StreamConsumer},
    producer::{FutureProducer, FutureRecord},
};

pub async fn produce(
    brokers: &str,
    topic: &str,
    key: Option<&String>,
    payload: Option<&String>,
    file: Option<&String>,
) -> anyhow::Result<()> {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()?;

    let key_str = key.map_or("", |k| k.as_str());

    let payload = if let Some(file_path) = file {
        println!("📄 Reading payload from file: {}", file_path);
        std::fs::read_to_string(file_path)?
    } else if let Some(p) = payload {
        println!("✉️  Using provided payload.");
        p.to_string()
    } else {
        String::new()
    };

    let delivery_status = producer
        .send(
            FutureRecord::to(topic).payload(&payload).key(key_str),
            Duration::from_secs(0),
        )
        .await;

    match delivery_status {
        Ok(delivery) => {
            println!(
                "✅ Sent! Partition: {}, Offset: {}",
                delivery.partition, delivery.offset
            )
        }
        Err((e, _)) => eprintln!("❌ Failed to sent: {:?}", e),
    }

    Ok(())
}

pub async fn list_topics(brokers: &str) -> anyhow::Result<()> {
    let base_consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .create()?;

    let metadata = base_consumer.fetch_metadata(None, Duration::from_secs(5))?;

    println!("📜 Available topics for {brokers}:");

    for topic in metadata.topics() {
        println!(" - {}", topic.name());
    }

    Ok(())
}

pub async fn consume(brokers: &str, group_id: &str, topic: &str) -> anyhow::Result<()> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .create()?;
    consumer.subscribe(&[topic])?;
    println!("📡 Listening topic '{}' in group '{}'...", topic, group_id);

    let mut message_stream = consumer.stream();

    while let Some(message) = message_stream.next().await {
        match message {
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(_)) => "<binary>",
                };

                let key = match m.key_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(_)) => "<binary>",
                };

                println!(
                    "Offset: {} | Key: {} | Payload: {}",
                    m.offset(),
                    key,
                    payload
                );

                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
            Err(e) => eprintln!("{e}"),
        }
    }

    Ok(())
}
