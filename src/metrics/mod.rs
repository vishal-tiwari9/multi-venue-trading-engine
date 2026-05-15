use prometheus::{Gauge,Histogram ,IntCounter,Registry};
use std::sync::OnceLock;

static REGISTRY:OnceLock<Regsitry>=OnceLock::new();

pub fn init_metrics() ->&'static Registry{
    REGISTRY.get_or_init(||{
        let registry = Registry::new();
        registry
    })
}


pub static MESSAGE_RECIEVED:OnceLock<IntCounter>=OnceLock::new();
pub static ORDERBOOK_UPDAT_LATENCY:OnceLock,Histogram>=OnceLock::new();
pub static ACTIVE_WEBSOCKETS:OnceLock<Gauge>=OnceLock::new();
(
pub fn register_metrics(){
    let regsitry = init_metrics();
    let mesaages =IntCounter::new("messages_REceived_total","Total messages received from exchanges")
    .unwrap();

    let latency = Histogram::new(
        "orderbook_update_latency_seconds",
        "Time Taken to Update Orderbook"
    ).unwrap();

    registry.register(Box::new(mesaages.clone()).unwrap());
    registry.register(Box::new(latency.clone())).unwrap();

    MESSGES_RECEIVED.set(mesaages);
    ORDERBOOK_UPDATE_LATENCY.set(latency);

}