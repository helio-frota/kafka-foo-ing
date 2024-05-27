use anyhow::Error;
use futures::executor::block_on;

use rdkafka::{
    admin::{AdminClient, AdminOptions, NewTopic},
    config::ClientConfig
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

fn main() -> Result<(), Error> {
    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", "localhost:9092");
    let topics = ["foo", "bar"];
    let future = create_topic(&topics, config);
    block_on(future)?;
    Ok(())
}
