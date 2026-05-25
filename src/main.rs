mod ledger;
mod market_data;
mod strategy;

use ledger::SimulationLedger;
use market_data::{MarketEvent, stream_binance, stream_hyperliquid};
use strategy::LocalOrderBook;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::unbounded_channel::<MarketEvent>();
    
    // Spawning dedicated asynchronous network processing routines
    tokio::spawn(stream_binance(tx.clone()));
    tokio::spawn(stream_hyperliquid(tx.clone()));

    let mut ledger = SimulationLedger::init("vault_state.json");
    let mut binance_book = LocalOrderBook::new();
    let mut hl_book = LocalOrderBook::new();

    let trade_lot_size = 5.0; 
    let fee_tier = 0.0010;    
    
    println!("Asynchronously pairing processing loops to streaming venues...");

    while let Some(event) = rx.recv().await {
        match event {
            MarketEvent::BinanceUpdate { bids, asks } => {
                binance_book.bids = bids;
                binance_book.asks = asks;
            }
            MarketEvent::HyperliquidUpdate { bids, asks } => {
                hl_book.bids = bids;
                hl_book.asks = asks;
            }
        }

        let binance_mid = binance_book.get_mid_price();
        let hl_mid = hl_book.get_mid_price();

        if binance_mid > 0.0 && hl_mid > 0.0 {
            let market_spread = (binance_mid - hl_mid).abs();
            ledger.update_net_worth(binance_mid);
            ledger.print_dashboard(binance_mid, hl_mid, market_spread);

            // Arbitrage Trigger Matrix Analysis Logic
            if binance_mid > hl_mid + 0.15 && ledger.state.usdc_balance > 1000.0 {
                let fill_price = hl_book.simulate_market_fill(true, trade_lot_size);
                if fill_price > 0.0 {
                    let fee = (trade_lot_size * fill_price) * fee_tier;
                    ledger.commit_trade(true, trade_lot_size, fill_price, fee);
                    println!(" -> [EXECUTION] Bought {:.2} SOL on Hyperliquid at slippage avg: ${:.2}", trade_lot_size, fill_price);
                }
            } else if hl_mid > binance_mid + 0.15 && ledger.state.sol_balance >= trade_lot_size {
                let fill_price = binance_book.simulate_market_fill(false, trade_lot_size);
                if fill_price > 0.0 {
                    let fee = (trade_lot_size * fill_price) * fee_tier;
                    ledger.commit_trade(false, trade_lot_size, fill_price, fee);
                    println!(" -> [EXECUTION] Sold {:.2} SOL on Binance at slippage avg: ${:.2}", trade_lot_size, fill_price);
                }
            }
        }
    }
}