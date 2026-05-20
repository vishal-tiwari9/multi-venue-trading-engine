use std::collections::BTreeMap;
use crate::types::{Price,Quantity,Side};
use super::level::OrderBookLevel;

#[derive(Debug)]
pub struct OrderBook{

    // Making Highest Price At the top thats why Reverse Ordered
pub bids:BTreeMap<Price,Quantity>, //Price -> Total Quantity at that Price 

pub asks:BTreeMap<Price,Quantity>,// Asks -> Sell Orders (Lowest Price is a first Priority)

pub last_update_time:u64,
pub exchange:String,




}

impl OrderBook{
    pub fn new(exchange:String)-> Self{
        
    }
}