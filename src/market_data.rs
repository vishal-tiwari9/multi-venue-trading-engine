use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

#[derive(Debug, Clone)]
pub enum MarketEvent {
    BinanceUpdate { bids: Vec<(f64, f64)>, asks: Vec<(f64, f64)> },
    HyperliquidUpdate { bids: Vec<(f64, f64)>, asks: Vec<(f64, f64)> },
}

pub async fn stream_binance(tx: UnboundedSender<MarketEvent>) {
    let url = Url::parse("wss://stream.binance.com:9443/ws/solusdt@depth5@100ms").unwrap();
    
    loop {
        // FIXED: Passed url.as_str() instead of &url to satisfy IntoClientRequest trait
        if let Ok((stream, _)) = connect_async(url.as_str()).await {
            let (_, mut read) = stream.split();
            while let Some(Ok(Message::Text(text))) = read.next().await {
                if let Ok(v) = serde_json::from_str::<Value>(&text) {
                    let mut bids = Vec::with_capacity(5);
                    let mut asks = Vec::with_capacity(5);

                    if let Some(b_arr) = v["b"].as_array() {
                        for item in b_arr.iter().take(5) {
                            let p = item[0].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
                            let q = item[1].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
                            bids.push((p, q));
                        }
                    }
                    if let Some(a_arr) = v["a"].as_array() {
                        for item in a_arr.iter().take(5) {
                            let p = item[0].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
                            let q = item[1].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
                            asks.push((p, q));
                        }
                    }
                    let _ = tx.send(MarketEvent::BinanceUpdate { bids, asks });
                }
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
}

pub async fn stream_hyperliquid(tx: UnboundedSender<MarketEvent>) {
    let url = Url::parse("wss://api.hyperliquid.xyz/ws").unwrap();
    
    loop {
        // FIXED: Passed url.as_str() instead of &url to satisfy IntoClientRequest trait
        if let Ok((mut stream, _)) = connect_async(url.as_str()).await {
            let sub_msg = r#"{"method": "subscribe", "subscription": {"type": "l2Book", "coin": "SOL"}}"#;
            if stream.send(Message::Text(sub_msg.to_string())).await.is_err() {
                continue;
            }
            
            let (_, mut read) = stream.split();
            while let Some(Ok(Message::Text(text))) = read.next().await {
                if let Ok(v) = serde_json::from_str::<Value>(&text) {
                    if v["channel"] == "l2Book" {
                        if let Some(levels) = v["data"]["levels"].as_array() {
                            let mut bids = Vec::with_capacity(5);
                            let mut asks = Vec::with_capacity(5);

                            if let Some(b_levels) = levels[0].as_array() {
                                for item in b_levels.iter().take(5) {
                                    let p = item["px"].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
                                    let q = item["sz"].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
                                    bids.push((p, q));
                                }
                            }
                            if let Some(a_levels) = levels[1].as_array() {
                                for item in a_levels.iter().take(5) {
                                    let p = item["px"].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
                                    let q = item["sz"].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
                                    asks.push((p, q));
                                }
                            }
                            let _ = tx.send(MarketEvent::HyperliquidUpdate { bids, asks });
                        }
                    }
                }
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
}