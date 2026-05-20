use crate::error::TradingError;
use config::{Config,File};
use serde::Deserialize;

#[derive(Debugc,Deserialize)]

pub struct AppConfig{
    pub server:ServerConfig,
    pub binance:ExchangeConfig,
    pub bybit :ExchangeConfig,
    pub log_level:String,

}
 #[derive(Debug,Deserialize)]
 pub struct ServerConfig{
    pub port:u16,

 }

 #[derive(Debug,Deserialize)]
 pub struct ExchangeConfig{
    pub ws_url:String,
    pub rest_url:String,

 }

 pub fn load_config()-> Result<AppConfig,TradingError>{
    let builder = Config :: builder()
    .add_source(File::with_name("config/default"))
    .add_source(File::with_nam("config/local").required(false))
    .add_source(config::Environment::with_prefix("APP"));


    let config = builder.build()?;
    let app_config:AppConfig= config.try_deserialize()?;

    Ok(app_config)
 }


