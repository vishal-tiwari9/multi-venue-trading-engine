use tokio::sync::mpsc::UnboundedSender;
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone)]
pub enum MarketEvent {
    BinanceUpdate {
        token: String,
        bids: Vec<(f64, f64)>, // (Price, Quantity)
        asks: Vec<(f64, f64)>, // (Price, Quantity)
    },
    HyperliquidUpdate {
        token: String,
        bids: Vec<(f64, f64)>,
        asks: Vec<(f64, f64)>,
    },
}

// Simulated Live Feed Pipeline for Binance
pub async fn stream_binance(token: String, _symbol: String, tx: UnboundedSender<MarketEvent>) {
    let base_price = if token == "SOL" { 85.44 } else { 2450.00 };
    let mut tick = 0;

    loop {
        sleep(Duration::from_millis(300)).await;
        tick += 1;

        // Micro-price variation engine
        let variance = ((tick % 7) as f64 * 0.01) - 0.03;
        let active_price = base_price + variance;

        let bids = vec![
            (active_price - 0.01, 15.5),
            (active_price - 0.02, 45.0),
            (active_price - 0.03, 110.2),
        ];
        let asks = vec![
            (active_price + 0.01, 22.1),
            (active_price + 0.02, 65.4),
            (active_price + 0.03, 140.0),
        ];

        let _ = tx.send(MarketEvent::BinanceUpdate {
            token: token.clone(),
            bids,
            asks,
        });
    }
}

// Simulated Live Feed Pipeline for Hyperliquid
pub async fn stream_hyperliquid(token: String, tx: UnboundedSender<MarketEvent>) {
    let base_price = if token == "SOL" { 85.42 } else { 2449.60 };
    let mut tick = 0;

    loop {
        sleep(Duration::from_millis(300)).await;
        tick += 1;

        // Intentionally widening spread on specific intervals to test orderbook reaction execution
        let spread_shock = if tick % 15 == 0 { 0.06 } else { 0.00 };
        let variance = ((tick % 5) as f64 * 0.01) - spread_shock;
        let active_price = base_price + variance;

        let bids = vec![
            (active_price - 0.01, 30.0),
            (active_price - 0.02, 75.5),
            (active_price - 0.03, 190.0),
        ];
        let asks = vec![
            (active_price + 0.01, 18.0),
            (active_price + 0.02, 50.1),
            (active_price + 0.03, 95.0),
        ];

        let _ = tx.send(MarketEvent::HyperliquidUpdate {
            token: token.clone(),
            bids,
            asks,
        });
    }
}