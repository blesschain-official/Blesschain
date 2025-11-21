📘 BlessChain – Testnet Development Branch (testnet-dev)

Next-Generation Home-Node Blockchain Network
Modular • Energy-Aware • Community-Powered

Branch: testnet-dev
Version: v0.2.x-devnet
Status: Active Runtime Development (Node temporarily disabled)


---

🚀 Overview

This branch hosts the Testnet Development (Devnet) version of BlessChain.

The current development phase focuses on:

Building & stabilizing the BlessChain Runtime

Verifying pallet configuration

Finalizing genesis configuration

Metadata & type correctness

Substrate SDK compatibility


⚠️ Important Note

The BlessChain Node does NOT build yet on this branch.
Only the runtime compiles successfully.
Full node support will be restored after completing runtime stabilization and SDK alignment.


---

🧱 Purpose of This Branch

✔ Runtime compilation

✔ Pallet integration testing

✔ Genesis & metadata validation

❌ Node execution (NOT supported yet)

✔ SDK compatibility testing

✔ Prepare for full testnet deployment



---

🏗 Project Structure (Corrected)

blesschain/
├── blesschain-node/
│   └── node/        ← node exists, but does NOT compile yet
│       ├── src/
│       └── Cargo.toml
│
├── blesschain-runtime/   ← CURRENT PRIMARY TARGET (compiles successfully)
│   ├── src/
│   └── Cargo.toml
│
├── pallets/
│   └── (custom pallets)
│
├── Cargo.toml
└── README.md

Status per component:

Component	Status

Runtime	✅ Builds successfully
Pallets	🟡 Under development, part of runtime
Node	❌ Temporarily NOT compiling
ChainSpec	❌ Not ready yet
SDK integration	🟡 In progress



---

🧩 BlessChain SDK (External Repository)

BlessChain depends on a patched Substrate SDK stored separately:

🔗 BlessChain SDK Repository:
https://github.com/blesschain-org/blesschain-sdk

Used for:

Offline builds

Patched Substrate modules

Deterministic compilation

wasm-opt / RocksDB / metadata fixes


Most developers do NOT need to modify it.
Runtime developers must clone it.


---

🛠 Build Instructions (Runtime Only)

1. Install toolchain

rustup update
rustup target add wasm32-unknown-unknown

2. Clone the SDK

git clone https://github.com/blesschain-org/blesschain-sdk

3. Clone BlessChain (testnet-dev)

git clone https://github.com/blesschain-org/blesschain
cd blesschain
git checkout testnet-dev


---

🧪 Build ONLY the Runtime (Correct Command)

BlessChain currently supports runtime-only compilation:

cargo build -p blesschain-runtime --release

or for debug:

cargo build -p blesschain-runtime

📌 Output:

WASM file located in:

blesschain-runtime/target/wasm32-unknown-unknown/release/*.wasm

⚠️ Node does NOT compile yet

You must NOT run:

cargo build

It will fail because:

node crate is unfinished

RPC and ChainSpec incomplete

SDK path updates still in progress



---

🏁 Node / Chain Execution

❌ Node execution is temporarily disabled on testnet-dev.

Future support will include:

./target/release/blesschain-node --dev

…but this will only work after node compilation is restored.


---

🔧 Features Under Active Development

Runtime

pallet_timestamp

pallet_balances

metadata fixes

genesis config

runtime API wiring


Node

Service builder cleanup

RPC integration

ChainSpec migration

Aura consensus bootstrapping


SDK

Correct module mapping

Vendor path realignment

wasm-builder-runner improvements



---

🧭 Branch Policy

Frequent updates

Breaking changes allowed

Runtime-first development

Node features re-enabled after runtime stabilization

PRs must target testnet-dev



---

🧪 How Developers Use This Branch

To build the runtime:

cargo build -p blesschain-runtime --release

To modify SDK internals:

git clone https://github.com/blesschain-org/blesschain-sdk


---

📩 Maintainer

Director: Joseph Wang
Organization: BlessChain Team
Website: https://blesschain.org

Contact via GitHub Issues or Pull Requests.


---

🏁 Final Notes

This README reflects the current real development status:

Runtime works

Node does NOT compile

Testnet-dev = Runtime stabilization phase

SDK must be cloned separately


