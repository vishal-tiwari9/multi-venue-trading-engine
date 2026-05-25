#[derive(Debug, Clone)]
pub struct LocalOrderBook {
    pub bids: Vec<(f64, f64)>, 
    pub asks: Vec<(f64, f64)>,
}

impl LocalOrderBook {
    pub fn new() -> Self {
        LocalOrderBook { bids: Vec::new(), asks: Vec::new() }
    }

    pub fn get_mid_price(&self) -> f64 {
        if self.bids.is_empty() || self.asks.is_empty() { return 0.0; }
        (self.bids[0].0 + self.asks[0].0) / 2.0
    }

    // Crawls down the orderbook array rows to track accurate liquidity depth cuts
    pub fn simulate_market_fill(&self, is_buy: bool, mut target_lot: f64) -> f64 {
        let matrix = if is_buy { &self.asks } else { &self.bids };
        let mut total_spent = 0.0;
        let mut filled = 0.0;

        for &(price, available_size) in matrix.iter() {
            if target_lot <= 0.0 { break; }
            let take_size = f64::min(target_lot, available_size);
            total_spent += take_size * price;
            filled += take_size;
            target_lot -= take_size;
        }

        if filled == 0.0 { 0.0 } else { total_spent / filled }
    }
}