use thiserror::Error;

#[derive(Error,Debug)]
pub enum TradingError{
    #[error("Exchange error:{0}")]
    Exchange(String),

 #[error("Websocket error:{0}")]
 WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

 #[error("JSON Parse error:{0}")]
   Json(#[from] serde_json::Error),

   #[error("Configuration error:{0}")]
   Config(#[from] config::ConfigError),

   #[error("OrderBook Error:{0}")]
   OrderBook(String),

   #[error("Risk Violation :{0}")]
   Risk(String),


}

pub type Result<T>= std::result::Result<T,TradingError>;