mod  error;
mod  types;

use tracing::info;
use crate::config::load_config;
use crate::tracing::init_tracing;


fn main()->Result<(),Box<dyn std::error::Error>>{
    
    init_tracing();
    info!("Application starting...");
    tracing_subscriber::fmt::init();

    let config = load_config()?;

  info!("Server port:{}",config.server.port);
  info!("Binance WS :{}",config.binance.ws_url);
    
    info!("Application Started Successfully");

    Ok(())
}