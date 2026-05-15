pub type Price= f64;
pub type Quantity = f64  ;

pub type Timestamp = u64;
pub type ExchangeName = String;

#[derive(Debug,Clone, PartialEq,Eq, PartialOrd,Ord,Hash)]
pub struct OrderBookLevel{
    pub price:Price,
    pub quantity:Quantity,

}

#[derive(Debug,Clone)]
pub enum Side{
    Bid,
    Ask
}

#[derive(Debug,Clone)]
pub struct Trade {
    pub price:Price,
    pub quantity:Quantity,
    pub side:Side,
    pub timestamp:Timstamp,
    pub exchange:ExchangeName,

}