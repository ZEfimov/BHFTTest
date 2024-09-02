//use std::{ collections::HashMap, convert::Infallible, sync::Arc };
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::stream::StreamExt;
use exchanges::Exchanges;
//use tokio::sync::{ mpsc, Mutex };
//use url::Url;
use clap::Parser;

mod exchanges;
mod book;
#[derive(Parser)]
struct Cli {
    #[clap(short, long, help = "(Optional) Currency pair to subscribe to. Default: btcusd")]
    symbol: Option<String>,

    #[clap(short, long, help = "(Optional) Update interval. Defaul 100ms")]
    interval: Option<usize>,

    #[clap(short, long, help = "(Optional) Book deep. Defaul 5")]
    deep: Option<usize>,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let interval: usize = args.interval.unwrap_or(1000);
    let deep: usize = args.deep.unwrap_or_else(||5);
    let sym_val = args.symbol.unwrap_or_else(|| "btcusdt".to_string());
    let symbols: Vec<&str> = sym_val.split(',').collect();
    let ex_name = Exchanges::Binance;
    let exchange = exchanges::get_exchange(ex_name);
    let url_result = exchange.generate_streams_url(&symbols, interval, deep);
    let url = match url_result {
        Ok(url) => url,
        Err(e) => {
            eprintln!("Failed to generate URL: {}", e);
            return;
        }
    };
    println!("Connect to url: {}", url);
    let (mut socket, response) = connect_async(url).await.expect("Failed to connect");

    //ToDo Add error handling
    while let Some(message) = socket.next().await {
        let message = message.unwrap();
        match message {
            Message::Text(text) => exchange.handle_message(text),
            Message::Binary(bin) => println!("Received binary data: {:?}", bin),
            _ => (),
        }
    }
}