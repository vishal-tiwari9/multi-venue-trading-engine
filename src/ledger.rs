use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use chrono::Utc;
use crate::strategy::LocalOrderBook;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VaultState {
    pub usdc_balance: f64,
    pub sol_balance: f64,
    pub eth_balance: f64,
    pub total_net_worth: f64,
}

pub struct SimulationLedger {
    file_path: String,
    pub state: VaultState,
    pub execution_logs: Vec<String>, // Tracks recent trades on dashboard
}

impl SimulationLedger {
    pub fn init(file_path: &str) -> Self {
        let path = Path::new(file_path);
        let state = if path.exists() {
            let mut file = File::open(path).expect("Failed to open vault state file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Failed to read ledger");
            serde_json::from_str(&contents).unwrap_or(Self::default_state())
        } else {
            let default = Self::default_state();
            let mut file = File::create(path).expect("Failed to create database mapping");
            let json = serde_json::to_string_pretty(&default).unwrap();
            file.write_all(json.as_bytes()).expect("Initial store crash");
            default
        };

        SimulationLedger {
            file_path: file_path.to_string(),
            state,
            execution_logs: Vec::new(),
        }
    }

    fn default_state() -> VaultState {
        VaultState {
            usdc_balance: 10000.0,
            sol_balance: 0.0,
            eth_balance: 0.0,
            total_net_worth: 10000.0,
        }
    }

    pub fn log_trade(&mut self, log: String) {
        self.execution_logs.push(log);
        if self.execution_logs.len() > 5 {
            self.execution_logs.remove(0); // Maintain last 5 actions
        }
    }

    pub fn commit_trade(&mut self, token: &str, is_buy: bool, amount: f64, execution_price: f64, fee: f64) {
        let total_cost = (amount * execution_price) + fee;
        if is_buy {
            if self.state.usdc_balance >= total_cost {
                self.state.usdc_balance -= total_cost;
                if token == "SOL" { self.state.sol_balance += amount; }
                else if token == "ETH" { self.state.eth_balance += amount; }
            }
        } else {
            let has_balance = if token == "SOL" { self.state.sol_balance >= amount }
                              else if token == "ETH" { self.state.eth_balance >= amount }
                              else { false };
            if has_balance {
                if token == "SOL" { self.state.sol_balance -= amount; }
                else if token == "ETH" { self.state.eth_balance -= amount; }
                let gross_revenue = amount * execution_price;
                self.state.usdc_balance += gross_revenue - fee;
            }
        }
        self.save_to_disk();
    }

    pub fn update_net_worth(&mut self, sol_price: f64, eth_price: f64) {
        let s_val = if sol_price > 0.0 { self.state.sol_balance * sol_price } else { 0.0 };
        let e_val = if eth_price > 0.0 { self.state.eth_balance * eth_price } else { 0.0 };
        self.state.total_net_worth = self.state.usdc_balance + s_val + e_val;
    }

    fn save_to_disk(&self) {
        let _ = File::create(&self.file_path).map(|mut f| {
            let json = serde_json::to_string_pretty(&self.state).unwrap();
            let _ = f.write_all(json.as_bytes());
        });
    }

    pub fn print_dashboard(
        &self, 
        sol_b: &LocalOrderBook, sol_hl: &LocalOrderBook, 
        eth_b: &LocalOrderBook, eth_hl: &LocalOrderBook
    ) {
        // Strict ANSI terminal clear code to prevent stacking loops
        print!("\x1B[2J\x1B[1;1H");
        
        let s_spread = (sol_b.get_mid_price() - sol_hl.get_mid_price()).abs();
        let e_spread = (eth_b.get_mid_price() - eth_hl.get_mid_price()).abs();

        println!("=================================================================================");
        println!("  DYNAMIC PORTFOLIO NET WORTH MANAGEMENT MATRIX                                  ");
        println!("  Net Worth: ${:<12.2} | USDC Balance: {:<10.2} | SOL: {:<7.2} | ETH: {:<7.2}", 
                 self.state.total_net_worth, self.state.usdc_balance, self.state.sol_balance, self.state.eth_balance);
        println!("=================================================================================");
        println!(" UTC Clock: {} | Systems Status: OPERATIONAL", Utc::now().format("%Y-%m-%d %H:%M:%S"));
        println!("---------------------------------------------------------------------------------");
        
        // --- SOLANA ORDERBOOK TABLE ENGINE ---
        println!(" [SOL/USDC MARKET PIPELINE]                                      Spread: ${:.3}", s_spread);
        println!("  +---------------------------------------+-------------------------------------+");
        println!("  |           BINANCE ORDER BOOK          |         HYPERLIQUID ORDER BOOK      |");
        println!("  +---------------------------------------+-------------------------------------+");
        println!("  |   Asks (Sells)    |    Bids (Buys)    |   Asks (Sells)    |    Bids (Buys)  |");
        println!("  +---------------------------------------+-------------------------------------+");
        
        for i in 0..3 {
            let b_ask = sol_b.asks.get(i).map(|x| format!("${:.2}", x.0)).unwrap_or_else(|| "--".to_string());
            let b_bid = sol_b.bids.get(i).map(|x| format!("${:.2}", x.0)).unwrap_or_else(|| "--".to_string());
            let hl_ask = sol_hl.asks.get(i).map(|x| format!("${:.2}", x.0)).unwrap_or_else(|| "--".to_string());
            let hl_bid = sol_hl.bids.get(i).map(|x| format!("${:.2}", x.0)).unwrap_or_else(|| "--".to_string());
            println!("  |  {:<16} | {:<17} | {:<16} | {:<14} |", b_ask, b_bid, hl_ask, hl_bid);
        }
        println!("  +---------------------------------------+-------------------------------------+");

        // --- ETHEREUM ORDERBOOK TABLE ENGINE ---
        println!("\n [ETH/USDC MARKET PIPELINE]                                      Spread: ${:.3}", e_spread);
        println!("  +---------------------------------------+-------------------------------------+");
        println!("  |           BINANCE ORDER BOOK          |         HYPERLIQUID ORDER BOOK      |");
        println!("  +---------------------------------------+-------------------------------------+");
        println!("  |   Asks (Sells)    |    Bids (Buys)    |   Asks (Sells)    |    Bids (Buys)  |");
        println!("  +---------------------------------------+-------------------------------------+");
        
        for i in 0..3 {
            let b_ask = eth_b.asks.get(i).map(|x| format!("${:.2}", x.0)).unwrap_or_else(|| "--".to_string());
            let b_bid = eth_b.bids.get(i).map(|x| format!("${:.2}", x.0)).unwrap_or_else(|| "--".to_string());
            let hl_ask = eth_hl.asks.get(i).map(|x| format!("${:.2}", x.0)).unwrap_or_else(|| "--".to_string());
            let hl_bid = eth_hl.bids.get(i).map(|x| format!("${:.2}", x.0)).unwrap_or_else(|| "--".to_string());
            println!("  |  {:<16} | {:<17} | {:<16} | {:<14} |", b_ask, b_bid, hl_ask, hl_bid);
        }
        println!("  +---------------------------------------+-------------------------------------+");

        // --- AUTOMATED TRADING TRANSACTION LOGS ---
        println!("\n RECENT SIMULATED ARBITRAGE REACTION LOGS:");
        if self.execution_logs.is_empty() {
            println!("  [IDLE] Scanning venues for spread expansion triggers...");
        } else {
            for log in &self.execution_logs {
                println!("  {}", log);
            }
        }
        println!("=================================================================================");
    }
}