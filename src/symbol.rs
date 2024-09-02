use crate::book::{OrderBook, Order};
use crate::exchanges::{Exchanges};
use std::collections::HashMap;
use rust_decimal::Decimal;

static Symbols: Lazy<Mutex<HashMap<Symbol, OrderBook>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    Mutex::new(m)
});

pub struct Symbol {
    name: String,
    exch: Exchanges
}
impl Symbol {
    pub fn new(name: String, exch: Exchanges)-> Self
    {
        Self {name, exch}
    }
}

pub fn get_symbol(stream: String, ex: Exchanges)->Symbol
{
  
}