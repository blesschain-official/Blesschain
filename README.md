✅ README.md for BlessChain v0.2.0 TestNet
BlessChain v0.2.0 TestNet
BlessChain v0.2.0 TestNet is the next stage of the BlessChain MVP —
 a lightweight, natively compiled blockchain that introduces configurable mock block production for test environments.
This repository contains the core node (blesschain-node) and runtime (blesschain-runtime) that together form the technical foundation for the Bless Ecosystem TestNet —
 powering experiments with validator nodes, token distribution, and AI-integrated services under real-time block simulation.

🚀 Key Features
✅ Fast, native-only blockchain (no WASM)
 ⚡ Mock block engine — produces blocks at a configurable interval (default 2 seconds)
 🔧 CLI control via --chain and --block-interval options
 💎 Minimal runtime including System, Balances, Aura, and Timestamp pallets
 🌐 Local Substrate SDK integration (~/blesschain-sdk)
 🧱 Fully native build (no wasm-builder or wasm-opt required)
 🧭 Compatible with Ubuntu 22.04 / 24.04 LTS
 🧩 Modular runtime design — ready for pallet extensions in future releases

🧰 Prerequisites
Component
Version / Notes
OS
Ubuntu 22.04 / 24.04 LTS
Rust
rustc 1.81+ (via rustup)
Cargo
Included with Rust
Toolchain
stable
Substrate SDK
Local clone at ~/blesschain-sdk
Build Target
Native (WASM disabled)


🧱 Environment Setup
Tested Environment
Install System Dependencies
sudo apt update
sudo apt install -y clang cmake make pkg-config libssl-dev git curl build-essential

Install Rust Toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup default stable
rustup target add wasm32-unknown-unknown

WASM target is optional — BlessChain TestNet builds natively.
Setup Local Directory Links
sudo mkdir -p /home/blesschain
sudo ln -s /home/$(whoami)/blesschain /home/blesschain/blesschain
sudo ln -s /home/$(whoami)/blesschain-sdk /home/blesschain/blesschain-sdk


🏗️ Build Instructions
1️⃣ Clone Repository
git clone https://github.com/blesschain-official/blesschain.git
cd blesschain

2️⃣ Clean Cache
cargo clean

3️⃣ Build Node (native-only)
cargo build --release -p blesschain-node --target-dir /mnt/data/blesschain-target

This compiles the BlessChain TestNet binary:
/mnt/data/blesschain-target/release/blesschain-node


▶️ Run the TestNet
🔹 Quick Start (default 2-second blocks)
/mnt/data/blesschain-target/release/blesschain-node

Starts the TestNet with:
chain = dev


block interval = 2 seconds


🔹 Advanced Mode (custom interval)
/mnt/data/blesschain-target/release/blesschain-node --chain dev --block-interval 7

Produces blocks every 7 seconds instead of 2.
Expected output:
🏗️  Starting BlessChain TestNet node...
⏱️  Producing mock blocks every 2 seconds
🧱  Imported #1 (0xabc...)
🧱  Imported #2 (0xdef...)


🧩 Project Structure
blesschain/
├── node/                    # Node service (main.rs, command.rs, service.rs)
├── blesschain-runtime/      # Runtime (System, Balances, Aura, Timestamp)
├── blesschain-sdk/          # Local Substrate SDK clone
├── local-crates/            # Local patches (e.g. wasm-builder-runner)
├── Cargo.toml               # Workspace definition
└── docs/                    # Developer documentation


⚙️ Validator Preparation (TestNet Mode)
BlessChain validators can join the TestNet to simulate block production and reward logic.
 A simplified setup is included for mock validators.
1️⃣ Generate Keys
/mnt/data/blesschain-target/release/blesschain-node key generate --scheme sr25519

Record the public key — this will be your validator identity.
2️⃣ Create Local Spec
/mnt/data/blesschain-target/release/blesschain-node build-spec > blesschain-testnet.json

3️⃣ Run Validator Node
/mnt/data/blesschain-target/release/blesschain-node \
  --chain blesschain-testnet.json \
  --block-interval 2

Future releases will integrate full Aura + Grandpa authority management.

🧠 Troubleshooting
Issue
Solution
frame/benchmarking/Cargo.toml missing
Remove frame-benchmarking dependency
sp-test-primitives not found
Add under [workspace.dependencies] or remove
Permission denied on /mnt/data/blesschain-target
Run:
sudo mkdir -p /mnt/data/blesschain-target && sudo chown $USER:$USER /mnt/data/blesschain-target
wasm-opt or wasm-builder errors
Remove wasm-related crates (native-only)
bandersnatch-experimental feature errors
Ensure local SDK matches the blesschain branch version


🧾 License
GPL-3.0-only — see LICENSE

🕊️ Credits
Developed by the BlessChain Team
 🌐 https://blesschain.com
 Project Director: Joseph Wang

📘 Version Notes
Date
Change
Notes
2025-10-20
First MVP build
2s block time on T7910
2025-11-11
Rebuilt on T7810
Native runtime confirmed









