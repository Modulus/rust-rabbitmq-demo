use amiquip::{Connection, Exchange, Publish, Result};


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
    // Open connection.
    let mut connection = Connection::insecure_open("amqp://kanin:kanin@localhost:5672")?;

    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;

    // Get a handle to the direct exchange on our channel.
    let exchange = Exchange::direct(&channel);

    // Publish a message to the "hello" queue.
    exchange.publish(Publish::new("hello there".as_bytes(), "hello"))?;

    connection.close()
}


