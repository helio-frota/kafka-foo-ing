use std::time::Duration;
use std::{thread, time};

use anyhow::Error;
use futures::executor::block_on;
use rdkafka::util::Timeout;

use rdkafka::TopicPartitionList;
use rdkafka::{
    admin::{AdminClient, AdminOptions, NewTopic},
    config::ClientConfig,
    consumer::{StreamConsumer, Consumer},
};

async fn create_topic(topics: &[&str], config: ClientConfig) -> Result<(), Error> {
    let admin: AdminClient<_> = config.create()?;
    let topics: Vec<NewTopic> = topics
        .iter()
        .map(|t| NewTopic::new(t, 1, rdkafka::admin::TopicReplication::Fixed(1)))
        .collect();
    admin
        .create_topics(&topics[..], &AdminOptions::default())
        .await?;
    Ok(())
}

fn topic_exists(config: ClientConfig, topic_name: &str, timeout: u64) -> Result<bool, Error> {
    let consumer: StreamConsumer = config.create()?;
    let out_time = Timeout::After(Duration::from_millis(timeout));
    let metadata = consumer.fetch_metadata(Some(topic_name), out_time)?;
    let topic_exists = metadata
        .topics()
        .iter()
        .any(|topic| topic.name() == topic_name);
    Ok(topic_exists)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", "localhost:9092");

   let topics = ["foo"];

    // match topic_exists(config.clone(), "foo", 3000) {
    //     Ok(exists) => {
    //         if exists {
    //             println!("yes {}", topics[0]);
    //         } else {
    //             println!("no {}", topics[0]);
    //         }
    //     },
    //     Err(e) => {
    //         eprintln!("Whatever error: {}", e);
    //     }
    // }

    let future = create_topic(&topics, config);
    block_on(future)?;
    println!("topic created -----------");

    println!("config consumer -----------");
    let mut config_consumer = ClientConfig::new();
    config_consumer
        .set("bootstrap.servers", "localhost:9092")
        .set("group.id", "foo-bar-group")
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "1000")
        .set("enable.auto.commit", "false")
        .set("auto.offset.reset", "earliest");

    println!("consumer create -----------");
    let c: StreamConsumer = config_consumer.create()?;

    println!("consumer sub -----------");
    c.subscribe(&topics[..])?;

    // println!("sleep time -----------");
    // let seconds = time::Duration::from_millis(10_000);
    // thread::sleep(seconds);

    let position = c.position()?;


    // let partition = 0;
    // let mut position = TopicPartitionList::new();
    // position.add_partition_offset(topics[0], partition, rdkafka::Offset::Offset(1))?;

    if position.count() > 0 {
        println!("consumer COMMIT -----------");
        Consumer::commit(&c, &position, rdkafka::consumer::CommitMode::Sync)?;
    }

    Ok(())
}
