# 🧱 BlessChain

**Next-Generation Home-Node Blockchain Network**

**Modular · Energy-Aware · Community-Powered**

BlessChain is a minimal proof-of-concept blockchain designed to evolve from **MVP → TestNet → MainNet**,  
focusing on home-server validator nodes and distributed edge infrastructure.

**Version:** v0.2.0 (MVP2.0)  
**Status:** MVP → TestNet Evolution  
**Author:** BlessChain Team  
**Project Director:** Joseph Wang  

🌐 **Website:** https://blesschain.org

📦 **Repository:** https://github.com/blesschain-official/Blesschain

---

## 📘 Blesschain Litepaper v0.1

Blesschain Litepaper v0.1 provides a high-level overview of the project vision, architectural principles, current development status, and roadmap.

This Litepaper is intended for community members, contributors, and external readers.  
It does **not** replace technical documentation or source code in this repository.

- Runtime layer: **Completed**
- Mock block execution: **Completed**
- Node & consensus integration: **In progress**

### 🌍 Language Support
The Litepaper is available in **13 languages**, including Simplified and Traditional Chinese.

### 📥 Download
- **GitHub Release:**  
  https://github.com/blesschain-official/Blesschain/releases/tag/v0.1-litepaper

---

📌 Overview

BlessChain is a lightweight Substrate-based blockchain designed to power the Bless Ecosystem — a decentralized network of:

AI video & image services

TTS / ASR services

Home-server validator nodes

Micro data-centers in hotels & homes

Token economy (BBTC) across all “Bless” products

Distributed compute + storage called HomeCDN


This repository contains:

blesschain-node — Rust node implementation

blesschain-runtime — Minimal FRAME runtime

Local Substrate SDK vendor — fully offline build (~/blesschain-sdk)

Genesis + chain_spec — dynamic mock block production


MVP2.0 (v0.2.0) is the first fully operational chain, verified on physical hardware.
---
🚀 Key Features

🔧 Minimal Native Runtime

pallet-system

pallet-balances

pallet-aura

pallet-timestamp


⚡ Dynamic Block Production

--block-interval 2    # Default 2 seconds
--block-interval 7    # Custom interval

🧩 Fully Native Build (No WASM)

No wasm-builder

No wasm-opt

No external GitHub downloads

100% controlled local vendor SDK


🏠 Optimized for Home Nodes

Tested on:

Dell R730XD

Dell T7910

Dell T7810
---

🧰 Prerequisites

Component	Version

OS	Ubuntu 22.04 / 24.04 LTS
Rust	rustc 1.81+
Cargo	Included with Rust
Toolchain	stable
Substrate SDK	~/blesschain-sdk
Build Target	Native only



---

📦 Install Build Requirements

sudo apt update
sudo apt install -y clang cmake make pkg-config libssl-dev git curl build-essential

Install Rust

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup default stable

(Optional WASM target)

rustup target add wasm32-unknown-unknown


---

📁 Create Standard BlessChain Directories

sudo mkdir -p /home/blesschain
sudo ln -s /home/$(whoami)/blesschain /home/blesschain/blesschain
sudo ln -s /home/$(whoami)/blesschain-sdk /home/blesschain/blesschain-sdk

---

🏗️ Build BlessChain (Native)

1️⃣ Clone

git clone https://github.com/blesschain-official/blesschain.git
cd blesschain

2️⃣ Clean

cargo clean

3️⃣ Build the Node

cargo build --release -p blesschain-node \
  --target-dir /mnt/data/blesschain-target

Final binary:

/mnt/data/blesschain-target/release/blesschain-node

---

▶️ Run the Local Development Chain

Default (2-second blocks)

/mnt/data/blesschain-target/release/blesschain-node

Custom block interval

/mnt/data/blesschain-target/release/blesschain-node \
  --chain dev \
  --block-interval 7

Example output:

🏗  Starting BlessChain Dev Node...
⏱  Producing mock blocks every 2 seconds
🧱  Imported #1 (0x....)
🧱  Imported #2 (0x....)

---

🧱 Project Structure

blesschain/
├── node/                     # Node code (main.rs, service.rs, command.rs)
├── blesschain-runtime/       # Runtime pallets
├── blesschain-sdk/           # Local Substrate SDK vendor
├── local-crates/             # Local patches (wasm-builder-runner etc.)
├── docs/                     # Whitepaper & architecture docs
└── Cargo.toml                # Workspace definition

---

🧭 Validator Setup (TestNet Simulation)

1️⃣ Generate Keys

/mnt/data/blesschain-target/release/blesschain-node key generate --scheme sr25519

2️⃣ Export chain spec

/mnt/data/blesschain-target/release/blesschain-node \
  build-spec > blesschain-testnet.json

3️⃣ Start validator

/mnt/data/blesschain-target/release/blesschain-node \
  --chain blesschain-testnet.json \
  --block-interval 2

---

🔍 Troubleshooting

Issue	Solution

frame-benchmarking errors	Remove benchmarking from runtime
sp-test-primitives missing	Add under [workspace.dependencies] or disable
/mnt/data permission denied	sudo mkdir -p + sudo chown $USER:$USER
wasm builder errors	Remove wasm-builder & wasm-opt
bandersnatch-experimental issues	Ensure blesschain-sdk matches runtime branch

---

📘 Version History

Version	Date	Notes

v0.1.0-mvp	2025-10-20	First runnable MVP
v0.1.1-devnet	2025-10-30	Minor patches
v0.2.0-mvp2.0	2025-11-11	Rebuilt node+runtime with full vendor SDK

---

📜 License

GPL-3.0-only — see LICENSE.


---

🕊 BlessChain Vision

BlessChain powers a decentralized ecosystem built around:

AI Video Generation (BlessAIVideo)

AI Voice (BlessVoice)

Image Hosting (BlessImage)

Video Platform (BlessVideo)

Booking (BlessBooking)

Hosting (BlessHosting)

Control Panel (BlessPanel)

Search Engine (BlessSearch)

Token Economy (BBTC)

HomeCDN distributed compute

MVP → TestNet → MainNet
This README represents the stable public version for the main branch.

🕊️ Credits
Developed by the BlessChain Team
 🌐 https://blesschain.org
 Project Director: Joseph Wang
