use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VaultState {
    pub usdc_balance: f64,
    pub sol_balance: f64,
    pub total_net_worth: f64,
}

pub struct SimulationLedger {
    file_path: String,
    pub state: VaultState,
}

impl SimulationLedger {
    pub fn init(file_path: &str) -> Self {
        let path = Path::new(file_path);
        let state = if path.exists() {
            let mut file = File::open(path).expect("Failed to open vault state file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Failed to read ledger string asset");
            serde_json::from_str(&contents).unwrap_or(Self::default_state())
        } else {
            let default = Self::default_state();
            let mut file = File::create(path).expect("Failed to instantiate database mapping");
            let json = serde_json::to_string_pretty(&default).unwrap();
            file.write_all(json.as_bytes()).expect("Initial execution store crash");
            default
        };

        SimulationLedger {
            file_path: file_path.to_string(),
            state,
        }
    }

    fn default_state() -> VaultState {
        VaultState {
            usdc_balance: 10000.0,
            sol_balance: 0.0,
            total_net_worth: 10000.0,
        }
    }

    pub fn commit_trade(&mut self, is_buy: bool, amount: f64, execution_price: f64, fee: f64) {
        if is_buy {
            let total_cost = (amount * execution_price) + fee;
            if self.state.usdc_balance >= total_cost {
                self.state.usdc_balance -= total_cost;
                self.state.sol_balance += amount;
            }
        } else {
            if self.state.sol_balance >= amount {
                let gross_revenue = amount * execution_price;
                self.state.usdc_balance += gross_revenue - fee;
                self.state.sol_balance -= amount;
            }
        }
        self.update_net_worth(execution_price);
        self.save_to_disk();
    }

    pub fn update_net_worth(&mut self, current_price: f64) {
        self.state.total_net_worth = self.state.usdc_balance + (self.state.sol_balance * current_price);
    }

    fn save_to_disk(&self) {
        let mut file = File::create(&self.file_path).expect("Failed ledger system write processing");
        let json = serde_json::to_string_pretty(&self.state).unwrap();
        file.write_all(json.as_bytes()).expect("Serial state validation writing dropped");
    }

    pub fn print_dashboard(&self, binance_p: f64, hl_p: f64, spread: f64) {
        // Multi-line refresh sequence executing ultra-clean fast screen clears
        print!("{}[2J{}[1;1H", 27 as char, 27 as char);
        println!("=============================================================");
        println!("         MULTI-VENUE LIQUIDITY & ARBITRAGE ENGINE            ");
        println!("         Timestamp: {} UTC", Utc::now().format("%Y-%m-%d %H:%M:%S"));
        println!("=============================================================");
        println!(" VENUE MARKET READINGS (SOL/USDC):");
        println!("   • Binance Live Price     : ${:.2}", binance_p);
        println!("   • Hyperliquid Live Price : ${:.2}", hl_p);
        println!("   • Gross Market Spread    : ${:.2}", spread);
        println!("-------------------------------------------------------------");
        println!(" PERSISTENT VAULT MOCK ACCOUNT LEDGER:");
        println!("   • Available Cash Balance : {:.2} USDC", self.state.usdc_balance);
        println!("   • Allocated Token Holdings: {:.4} SOL", self.state.sol_balance);
        println!("   • Dynamic Vault NetWorth : ${:.2}", self.state.total_net_worth);
        println!("=============================================================");
    }
}