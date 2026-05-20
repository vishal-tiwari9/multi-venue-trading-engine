mod  error;
mod  types;
mod config ;
mod tracing;
mod metrics;


use tracing::info;
use crate::config::load_config;
use crate::tracing::init_tracing;
use crate::metrics::register_metrics;


fn main()->Result<(),Box<dyn std::error::Error>>{
    
   info!("Adding Prometheus Metrics");
// Load Tracing
    init_tracing();
    info!("Application starting...");
    tracing_subscriber::fmt::init();
 // Load Config 
    let config = load_config()?;

  info!("Server port:{}",config.server.port);
  info!("Binance WS :{}",config.binance.ws_url);
    
    info!("Application Started Successfully");

    Ok(())
}