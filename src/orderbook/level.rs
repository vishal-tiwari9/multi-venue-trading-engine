// Level represnt the single price level 


use create::types::{Price,Quantity};

#[derive(Debug,Clone)]

pub struct OrderBookLevel{
    pub price :Price,
    pub quantity:Quantity,

}

impl OrderBookLevel{
    pub fn new(price:Price,quantity:Quantity)->Self{
        Self{price,quantity}
    }

    pub fn is_empty(&self)->bool{
        self.quantity<=0.0
    }
}