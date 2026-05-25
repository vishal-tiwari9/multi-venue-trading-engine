mod ledger;
mod market_data;
mod strategy;

use ledger::SimulationLedger;
use market_data::{MarketEvent, stream_binance, stream_hyperliquid};
use strategy::LocalOrderBook;
use tokio::sync::mpsc;
use chrono::Utc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::unbounded_channel::<MarketEvent>();
    
    // Spawn pipelines
    tokio::spawn(stream_binance("SOL".to_string(), "solusdt".to_string(), tx.clone()));
    tokio::spawn(stream_hyperliquid("SOL".to_string(), tx.clone()));
    tokio::spawn(stream_binance("ETH".to_string(), "ethusdt".to_string(), tx.clone()));
    tokio::spawn(stream_hyperliquid("ETH".to_string(), tx.clone()));

    let mut ledger = SimulationLedger::init("vault_state.json");
    
    let mut sol_binance = LocalOrderBook::new();
    let mut sol_hl = LocalOrderBook::new();
    let mut eth_binance = LocalOrderBook::new();
    let mut eth_hl = LocalOrderBook::new();

    let sol_lot = 2.0; 
    let eth_lot = 0.2;
    let fee_tier = 0.0010;    

    while let Some(event) = rx.recv().await {
        match event {
            MarketEvent::BinanceUpdate { token, bids, asks } => {
                if token == "SOL" { sol_binance.bids = bids; sol_binance.asks = asks; }
                else if token == "ETH" { eth_binance.bids = bids; eth_binance.asks = asks; }
            }
            MarketEvent::HyperliquidUpdate { token, bids, asks } => {
                if token == "SOL" { sol_hl.bids = bids; sol_hl.asks = asks; }
                else if token == "ETH" { eth_hl.bids = bids; eth_hl.asks = asks; }
            }
        }

        let s_b_mid = sol_binance.get_mid_price();
        let s_h_mid = sol_hl.get_mid_price();
        let e_b_mid = eth_binance.get_mid_price();
        let e_h_mid = eth_hl.get_mid_price();

        ledger.update_net_worth(s_b_mid, e_b_mid);
        
        // Print the real-time matrix layout dashboard
        ledger.print_dashboard(&sol_binance, &sol_hl, &eth_binance, &eth_hl);

        // --- AUTOMATED TRIGGER CHECK FOR SOL (Lowered to $0.02) ---
        if s_b_mid > 0.0 && s_h_mid > 0.0 {
            if s_b_mid > s_h_mid + 0.02 && ledger.state.usdc_balance > 500.0 {
                let fill = sol_hl.simulate_market_fill(true, sol_lot);
                if fill > 0.0 {
                    let fee = (sol_lot * fill) * fee_tier;
                    ledger.commit_trade("SOL", true, sol_lot, fill, fee);
                    ledger.log_trade(format!("[{}] -> EXECUTION: Bought {:.2} SOL on HL at ${:.2}", Utc::now().format("%H:%M:%S"), sol_lot, fill));
                }
            } else if s_h_mid > s_b_mid + 0.02 && ledger.state.sol_balance >= sol_lot {
                let fill = sol_binance.simulate_market_fill(false, sol_lot);
                if fill > 0.0 {
                    let fee = (sol_lot * fill) * fee_tier;
                    ledger.commit_trade("SOL", false, sol_lot, fill, fee);
                    ledger.log_trade(format!("[{}] -> EXECUTION: Sold {:.2} SOL on Binance at ${:.2}", Utc::now().format("%H:%M:%S"), sol_lot, fill));
                }
            }
        }

        // --- AUTOMATED TRIGGER CHECK FOR ETH (Lowered to $0.20) ---
        if e_b_mid > 0.0 && e_h_mid > 0.0 {
            if e_b_mid > e_h_mid + 0.20 && ledger.state.usdc_balance > 1000.0 {
                let fill = eth_hl.simulate_market_fill(true, eth_lot);
                if fill > 0.0 {
                    let fee = (eth_lot * fill) * fee_tier;
                    ledger.commit_trade("ETH", true, eth_lot, fill, fee);
                    ledger.log_trade(format!("[{}] -> EXECUTION: Bought {:.2} ETH on HL at ${:.2}", Utc::now().format("%H:%M:%S"), eth_lot, fill));
                }
            } else if e_h_mid > e_b_mid + 0.20 && ledger.state.eth_balance >= eth_lot {
                let fill = eth_binance.simulate_market_fill(false, eth_lot);
                if fill > 0.0 {
                    let fee = (eth_lot * fill) * fee_tier;
                    ledger.commit_trade("ETH", false, eth_lot, fill, fee);
                    ledger.log_trade(format!("[{}] -> EXECUTION: Sold {:.2} ETH on Binance at ${:.2}", Utc::now().format("%H:%M:%S"), eth_lot, fill));
                }
            }
        }
    }
}