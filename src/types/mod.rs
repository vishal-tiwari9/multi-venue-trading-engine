pub type Price:f64;
pub type Quantity :f64;
pub type Timestamp:u64;

#[derive(Debug,Clone, PartialEq,Eq, PartialOrd,Ord,Hash)]

struct OrderBookLevel{
    pub price:Price,
    pub quantity:Quantity,

}

#[derive(Debug,Clone)]
pub enum Side{
    Bid,
    Ask
}

#[derive(Debug,Clone)]
pub enum Trade{
    pub price:Price,
    pub quantity:Quantity,
    pub side:Side,
    pub timestamp:Timstamp,

}