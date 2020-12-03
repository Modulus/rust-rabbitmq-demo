use amiquip::{Connection, Exchange, Publish, Result};
// use rand::seq::SliceRandom; // 0.7.2

use std::env;
use names::{Generator, Name};
use std::{thread, time};


// use serde::Deserialize;
// use reqwest::Error;
//
// #[derive(Deserialize, Debug)]
// struct Data {
//     data: Vec<String>
// }
//
// fn main() {
//     let lorem_url = format!("http://loremricksum.com/api/?paragraphs={paragraphs}&quotes={quotes}", paragraphs=1, quotes=1);
//
//     println!("Calling url: {:?}", lorem_url);
//
//     let response = reqwest::blocking::get(&lorem_url).unwrap().text();
//
//     println!("{:?}", response);
//
//     let data: Vec<String> = response.json();
//
//
// }
fn main() -> Result<()> {

    let rabbitmq_url : String = env::var("RABBITMQ_CONNECTION_STRING").unwrap_or("amqp://kanin:kanin@localhost:5672".to_string());

    println!("Calling rabbitmq on {:?}", rabbitmq_url);

    let mut generator = Generator::with_naming(Name::Numbered);

    // Routing keys
    // let routing_keys = ["", "hello", "hi", "random", "1239213", "asdfg"];

    loop {
        // Open connection.

        let mut connection = Connection::insecure_open(&rabbitmq_url)?;

        // Open a channel - None says let the library choose the channel ID.
        let channel = connection.open_channel(None)?;

        // Get a handle to the direct exchange on our channel.
        let exchange = Exchange::direct(&channel);

        let name = generator.next().unwrap();
        println!("current name is: {}", name);

        // let routing_key: Vec<_> = routing_keys
        //     .choose_multiple(&mut rand::thread_rng(), 1)
        //     .collect();
        // println!("Current routing key{:?}", routing_key);
        //
        // Publish a message to the "hello" queue.
        exchange.publish(Publish::new(name.as_bytes(), "hello"))?;

        connection.close();
        let ten_millis = time::Duration::from_secs(2);

        thread::sleep(ten_millis);

    }

    println!("Done!");


}


