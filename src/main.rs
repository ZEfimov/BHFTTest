use clap::Parser;
use exchanges::Exchanges;
use futures_util::stream::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

mod book;
mod exchanges;
#[derive(Parser)]
struct Cli {
    #[clap(
        short,
        long,
        help = "(Optional) Currency pair to subscribe to. Default: btcusdt"
    )]
    symbol: Option<String>,

    #[clap(short, long, help = "(Optional) Update interval. Default 1000ms")]
    interval: Option<usize>,

    #[clap(short, long, help = "(Optional) Book deep. Default 5")]
    deep: Option<usize>,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let interval: usize = args.interval.unwrap_or(1000);
    let deep: usize = args.deep.unwrap_or(5);
    let sym_val = args
        .symbol
        .unwrap_or_else(|| "btcusdt".to_string())
        .to_lowercase();
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
    for (key, value) in response.headers() {
        println!("{}: {:?}", key, value);
    }
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
