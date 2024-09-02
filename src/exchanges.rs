use url::Url;
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::str::FromStr;

use crate::book::{OrderBook, Order};

pub enum Exchanges {
    Binance
}

trait PrivateBaseExchange
{
    fn get_ws_url(&self) -> String;
}
pub trait BaseExchange
{
    fn generate_streams_url(&self, symbols: &[&str], update_timeout: usize, deep: usize) -> Result<Url, url::ParseError>;
    fn handle_message(&self, text: String);
}
#[derive(Debug, Serialize, Deserialize)] 
struct BinanceUpdateData {
    lastUpdateId: u64,
    bids: Vec<(String, String)>,
    asks: Vec<(String, String)>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BinanceStreamData {
    stream: String,
    data: BinanceUpdateData,
}
#[derive(Debug)]
struct Binance;

impl PrivateBaseExchange for Binance
{
    fn get_ws_url(&self) -> String
    {
        "wss://stream.binance.com:9443".to_string()
    }
}

impl BaseExchange for Binance
{
    fn handle_message(&self, text: String)
    {
        let mut order_book = OrderBook::new();
        let message: BinanceStreamData = match serde_json::from_str(&text) {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("Failed to parse message: {}", e);
                return;
            }
        };
        order_book.last_update_id = message.data.lastUpdateId;
        //ToDo Fix code duplication
        for bid in message.data.bids 
        {
            let price = Decimal::from_str(&bid.0).unwrap();
            let quantity = Decimal::from_str(&bid.1).unwrap();
            order_book.add_order(Order { price, quantity }, crate::book::OrderType::BID);
        }

        for ask in message.data.asks 
        {
            let price = Decimal::from_str(&ask.0).unwrap();
            let quantity = Decimal::from_str(&ask.1).unwrap();
            order_book.add_order(Order { price, quantity }, crate::book::OrderType::ASK);
        }
        println!("{}",message.stream);
        order_book.prinst_statistic();
    }

    fn generate_streams_url(&self, symbols:&[&str], update_timeout: usize, deep: usize) -> Result<Url, url::ParseError>
    {
        let streams = symbols
        .iter()
        .map(|sym| format!("{}@depth{}@{}ms", sym, deep, update_timeout))
        .collect::<Vec<_>>()
        .join("/");
        let url_str = format!("{}/stream?streams={}", self.get_ws_url(), streams);
        Url::parse(&url_str)
    }
}

pub fn get_exchange(ex: Exchanges) -> Box<dyn BaseExchange>
{
    match ex {
        Exchanges::Binance => Box::new(Binance),
        _ => panic!("Unknown Exchange"),
    }
}