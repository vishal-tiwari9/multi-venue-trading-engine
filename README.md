

# Multi-Venue Trading Engine (SOL & ETH Arbitrage Pipeline)

A high-performance, low-latency, asynchronous event-driven market liquidity and arbitrage engine engineered in **Rust**. This system concurrently streams real-time orderbook depth layers from multiple venues (**Binance** and **Hyperliquid**), computes micro-price anomalies across assets (**SOL** and **ETH**), executes virtual multi-tiered market fills, and updates a state-persistent cryptographic simulation ledger.

---
CLI 

<img width="599" height="439" alt="image" src="https://github.com/user-attachments/assets/a6e89472-90da-4646-a19e-2655c48696cf" />
---


##  Key Architectural Features & Tech Stack

###  Tech Stack Used

* **Language:** Rust (Stable, 2021 Edition)
* **Async Runtime:** `tokio` (Multi-threaded, macro-driven asynchronous execution scheduler)
* **Networking Protocol:** `tokio-tungstenite` (Asynchronous WebSockets protocol implementation over TLS)
* **Serialization Pipeline:** `serde` & `serde_json` (Zero-copy data de/serialization data pipeline)
* **State Auditing:** `chrono` (Sub-millisecond UTC atomic timestamp sequencing)

###  Design Pattern: Asynchronous MPSC Event-Driven Architecture

The engine is constructed using a decoupled **Multi-Producer Single-Consumer (MPSC)** channel workflow.

* **The Ingestors (Producers):** Isolated concurrent green threads handle heavy Network I/O and parse WebSocket packages without blocking computing routines.
* **The Processor (Consumer):** A centralized event loop receives structured cross-venue events, modifies in-memory depth orderbooks, checks immediate arbitrage spreads, and handles state mutations.
* **Zero Mutex Contention:** By using communication channels instead of shared-state memory pointers (`Arc<Mutex<T>>`), the core loop guarantees race-condition free data mutations at maximum compute velocity.

---

##  Repository Layout & File Descriptions

```
├── Cargo.toml               # Cargo package manager configuration & optimizations
└── src
    ├── main.rs              # Central Orchestrator & Strategy Evaluator
    ├── market_data.rs       # Network Layer / Async Exchange WebSocket Streams
    ├── strategy.rs          # Data Memory Layer / Book depth & execution logic
    └── ledger.rs            # Accounting Layer / JSON State Auditor & Terminal UI

```

### 1. `src/market_data.rs` — The Data Pipe

Responsible for lifecycle handshakes with remote exchange clusters.

* Establishes asynchronous WebSocket nodes using native TLS wrappers.
* Maps raw network bytes into structural Rust-native memory data arrays (`MarketEvent` enum variants).
* Implements an autonomous self-healing connection loop with structured network error-drop backoffs.

### 2. `src/strategy.rs` — The Orderbook Core

Maintains local structural synchronization grids of active order levels.

* Tracks depth up to 5 multi-tiered positions of Ask (Sell) and Bid (Buy) limits.
* Computes asset mathematical equilibrium mid-points dynamically via:

$$\text{Mid Price} = \frac{\text{Best Ask} + \text{Best Bid}}{2}$$


* Contains a Volume-Weighted Average Price (VWAP) slip calculator (`simulate_market_fill`) to map historical slippage metrics if an instantaneous macro-size market-order hits the queue layers.

### 3. `src/ledger.rs` — The Financial Auditor

Manages account profile structures, ledger transaction books, and terminal rendering layout.

* **Double-Entry Mock System:** Tracks total cash reserves (`USDC`), active token inventories (`SOL` & `ETH`), and continuously updates total asset portfolio wealth evaluations (`Total Net Worth`).
* **Crash-Proof Persistence:** Serializes account states to a physical flat file asset array named `vault_state.json`. If the processing daemon suffers unexpected force shutdowns, initialization sequences recover identical balances.
* **Static ANSI Dashboard Engine:** Utilizes atomic UI cursor flush operations (`\x1B[2J\x1B[1;1H`) to output clean side-by-side trading desk matrices directly into your terminal without layout vertical tearing.

### 4. `src/main.rs` — The Brain & Trigger Matrix

The centralized processing hub. Spawns the worker threads, channels incoming feeds to correct target parameters, monitors price spreads against trigger boundaries ($0.02 for SOL, $0.20 for ETH), and triggers automated orders when a profitable cross-venue delta is uncovered.

---

## ⚙️ How It Works (End-to-End Pipeline Lifecycle)

```
[WebSocket Pipelines] -> (Spit MarketEvent) -> [MPSC Core Receiver] 
                                                         |
                                                         v
                                              [Update Local Books]
                                                         |
                                                         v
                                              [Recalculate Net Worth]
                                                         |
                                                         v
                                            [Evaluate Spread Matrix]
                                              /                    \
                     (Spread > Threshold) ---/                      \--- (Spread <= Threshold)
                             |                                                   |
                             v                                                   v
         [Simulate Slippage & Execute Trade]                             [Render UI Dashboard]
                             |
                             v
           [Persist JSON to Disk Vault]

```

1. **Ingestion:** Separate network worker pipelines spawn for SOL and ETH on Binance and Hyperliquid. They parse streaming text streams directly into an optimal memory representation.
2. **Aggregation:** The primary thread accepts incoming memory structures, instantly allocating the specific venue's ask/bid boundaries to their distinct memory struct.
3. **Arbitrage Audit:** The strategy layer verifies if conditions are ripe for cross-exchange trading. For example:
* If **Binance Mid Price > Hyperliquid Mid Price + Spread Threshold**, it routes an instant order routine to **Buy Cheap on Hyperliquid** and **Sell Premium on Binance**.


4. **Risk Clearance:** The engine asserts cash validation bounds. If USDC balances clear risk limits, it computes execution fills, logs the receipt, and decreases/increases internal inventories.
5. **Persistence & UI Layout Render:** Account values write directly to disk, and the terminal layout re-draws tabular components detailing current balances and tracking statistics.

---

## 🛠️ Installation & Execution Guide

### Prerequisites

* Ensure the [Rust Toolchain (Cargo Compiler)](https://www.rust-lang.org/tools/install) is configured.

### Clone the Repository

```bash
git clone https://github.com/vishal-tiwari9/multi-venue-trading-engine.git
cd multi-venue-trading-engine

```

### Development Testing (Unoptimized Logs Mode)

```bash
cargo run

```

### Production Mode Execution (Highly Optimized Native Binaries)

To bypass Rust's internal debug safety assertions and activate compiler level 3 graph optimizations (`opt-level = 3`), run with the release configuration:

```bash
cargo run --release

```

---

## 🤝 Contribution Guidelines

Contributions are highly welcome! To implement feature proposals, optimize the pricing matrix loops, or design live websocket connectors, please follow these steps:

1. **Fork** the project repository.
2. Create your feature branch (`git checkout -b feature/AmazingQuantUpgrade`).
3. Commit your atomic shifts with high-clarity notes (`git commit -m 'feat: optimized book depth search routines'`).
4. Push your changes to your remote branch repository (`git push origin feature/AmazingQuantUpgrade`).
5. Open a **Pull Request** explaining your code patches, and ensure your code compiles clean under `cargo clippy` lint rules!

