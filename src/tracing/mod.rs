use tracing_subscriber::{layer::SubscriberExt,util::SubscriberInirExt,EnvFilter};


pub fn init_tracing(){
    let filter = EnvFilter::from_default_env()
    .add_directive("Multi_Venue_trading_engine=info".parse().unwrap());

   tracing_subscriber::registry()
   .with(filter)
   .with(tracing_subscriber::fmt::layer())
   .init();
}