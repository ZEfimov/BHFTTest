use std::collections::BinaryHeap;
use rust_decimal::Decimal;
use std::cmp::Ordering;
use std::cmp::Reverse;

pub enum OrderType 
{
    BID,
    ASK
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    pub price: Decimal,
    pub quantity: Decimal,
}

impl Ord for Order {
    fn cmp(&self, other: &Self) -> Ordering {
        self.price.partial_cmp(&other.price).unwrap()
    }
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct OrderBook {
    pub last_update_id: u64,
    pub bids: BinaryHeap<Order>,
    pub asks: BinaryHeap<Reverse<Order>>
}

impl OrderBook 
{
    pub fn new() -> Self 
    {
        OrderBook 
        {
            last_update_id: 0,
            bids: BinaryHeap::new(),
            asks: BinaryHeap::new(),
        }
    
    
    }
    pub fn clear_books(&mut self) 
    {
        self.clear_book(OrderType::ASK);
        self.clear_book(OrderType::BID);
    }

    pub fn clear_book(&mut self, order_type: OrderType) 
    {
        match order_type 
        {
            OrderType::BID => self.bids.clear(),
            OrderType::ASK => self.asks.clear(),
        }
    }
    pub fn add_order(&mut self, order: Order, order_type: OrderType) 
    {
        match order_type 
        {
            OrderType::BID => self.bids.push(order),
            OrderType::ASK => self.asks.push(Reverse(order)),
        }
    }
    fn get_best_ask(&self)-> Option<&Order>
    {
        self.asks.peek().map(|reverse_order| &reverse_order.0)
    }
    fn get_best_bid(&self)-> Option<&Order>
    {
        self.bids.peek()
    }
    fn get_wap_asks(&self)-> Decimal
    {
        let mut w_sum = Decimal::new(0, 0);
        let mut qx_sum = Decimal::new(0, 0);
    
        for reverse_order  in self.asks.iter() 
        {
                let order = &reverse_order.0;
                w_sum += order.price * order.quantity;
                qx_sum += order.quantity;    
        }
    
        if qx_sum.is_zero() {
            Decimal::new(0, 0)
        } else {
            let r = w_sum / qx_sum;
            r.round_dp(4)
        }
    }
    fn get_wap_bids(&self)-> Decimal
    {
        let mut w_sum = Decimal::new(0, 0);
        let mut qx_sum = Decimal::new(0, 0);
    
        for order in self.bids.iter() 
        {
                w_sum += order.price * order.quantity;
                qx_sum += order.quantity;    
        }
    
        if qx_sum.is_zero() {
            Decimal::new(0, 0)
        } else {
            let r = w_sum / qx_sum;
            r.round_dp(4)
        }
    }
    pub fn prinst_statistic(&self)
    {
        println!("BestBid - {:?}\nBestAsk - {:?}\nBid WAP - {:?}\nAsk WAP - {:?}",
                self.get_best_bid(), self.get_best_ask(), 
                self.get_wap_bids(), self.get_wap_asks())
    }
}
    