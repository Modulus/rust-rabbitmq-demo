use serde::Deserialize;
use reqwest::Error;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Data {
    success: Success,
    contents: Contents,
    baseurl: String,
    copyright: Copyright

}

#[derive(Debug, Deserialize)]
struct Success {
    total: i32
}

#[derive(Debug, Deserialize)]
struct Contents {
    quotes: Vec<Quotes>
}

#[derive(Debug, Deserialize)]
struct Quotes  {
    quote: String,
    length: String,
    author: String,
    tags: Vec<String>,
    category: String,
    language: String,
    date: String,
    permalink: String,
    id: String,
    background: String,
    title: String
}

#[derive(Debug, Deserialize)]
struct Copyright {
    year: u32,
    url: String
}


#[tokio::main]
async fn main()  -> Result <(), Box<dyn std::error::Error>> {
    let quote = get_quote().await?;
    println!("{:?}", quote.contents.quotes);

    Ok(())

}

async fn get_quote() -> Result <(Data), Box<dyn std::error::Error>> { //HashMap<String, String>
    let quotes_url = format!("https://quotes.rest/qod");


    println!("Calling url: {:?}", quotes_url);

    let response = reqwest::get(&quotes_url).await?
    .json::<Data>()
    .await?;

    println!("{:?}", response);

    Ok((response))
}