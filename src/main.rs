use std::time::Duration;

use anyhow::Error;
use futures::executor::block_on;
use rdkafka::util::Timeout;

use rdkafka::{
    admin::{AdminClient, AdminOptions, NewTopic},
    config::ClientConfig,
    consumer::{BaseConsumer, Consumer},
};

async fn create_topic(topics: &[&str], config: ClientConfig) -> Result<(), Error> {
    let admin: AdminClient<_> = config.create()?;
    let topics: Vec<NewTopic> = topics
        .iter()
        .map(|t| NewTopic::new(t, 1, rdkafka::admin::TopicReplication::Fixed(1)))
        .collect();
    admin.create_topics(&topics[..], &AdminOptions::default()).await?;
    Ok(())
}

fn topic_exists(config: ClientConfig, topic_name: &str, timeout: u64) -> Result<bool, Error> {
    let consumer: BaseConsumer = config.create()?;
    let out_time = Timeout::After(Duration::from_millis(timeout));
    let metadata = consumer.fetch_metadata(Some(topic_name), out_time)?;
    let topic_exists = metadata.topics().iter().any(|topic| topic.name() == topic_name);
    Ok(topic_exists)
}

fn main() -> Result<(), Error> {
    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", "localhost:9092");
    let topics = ["foo"];

    match topic_exists(config.clone(), "foo", 3000) {
        Ok(exists) => {
            if exists {
                println!("yes {}", topics[0]);
            } else {
                println!("no {}", topics[0]);
            }
        },
        Err(e) => {
            eprintln!("Whatever error: {}", e);
        }
    }

    let future = create_topic(&topics, config);
    block_on(future)?;

    Ok(())
}
