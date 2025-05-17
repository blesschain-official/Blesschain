# 🧪 How to Run BlessChain Local Testnet

This guide walks you through building and launching the **BlessChain** development network (testnet) from source.

---

## ⚙️ Prerequisites

- Rust with nightly toolchain
- `wasm32-unknown-unknown` target
- Git + Cargo
- Ubuntu/Debian/Linux environment

---

## 📥 1. Clone the Repository

```bash
git clone https://github.com/blesschain-org/blesschain
cd blesschain
```

---

## 🔧 2. Build the Node

> Note: This will use the local `.cargo/config.toml` to output into `/mnt/data/blesschain-target`

```bash
cargo +nightly build --release
```

Resulting binary path:

```
/mnt/data/blesschain-target/release/blesschain-node
```

---

## 🧬 3. Launch the Testnet (dev mode)

```bash
./mnt/data/blesschain-target/release/blesschain-node --dev --base-path /mnt/data/blesschain-data
```

This will start a single-node development chain with:
- Dynamic block time (2s to 7s)
- BBTC native token
- Preconfigured accounts

---

## 📦 4. Genesis Configuration (optional)

You may load a custom chain spec using:
```bash
./blesschain-node build-spec --dev > blesschain-dev-plain.json
./blesschain-node build-spec --dev --raw > blesschain-dev-raw.json
```

To launch with your chain spec:
```bash
./blesschain-node --chain blesschain-dev-raw.json --base-path /mnt/data/blesschain-data
```

---

## 📡 5. RPC and Explorer (coming soon)

RESTful APIs and light explorer dashboard are planned in v0.3+.

---

## 🙏 Troubleshooting

- Make sure Rust is set to nightly: `rustup default nightly`
- Make sure you installed wasm target: `rustup target add wasm32-unknown-unknown`
- Ensure permissions: `chmod +x ./blesschain-node`
- Use `tmux` to keep node running in background

---

Blessings! ✝️
