// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.
//! Minimal RPC wiring for BlessChain Node.
//! No polkadot_sdk, no runtime APIs, no WASM RPCs.

//! Minimal RPC wiring for BlessChain Node.

use std::sync::Arc;
use jsonrpsee::RpcModule;

/// Full client dependencies (kept for future extension).
pub struct FullDeps<C, P> {
    /// The client instance to use.
    pub client: Arc<C>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
}

/// Instantiate all full RPC extensions.
///
/// Currently returns an empty `RpcModule`, so the node starts but exposes no RPC methods yet.
pub fn create_full<C, P>(
    _deps: FullDeps<C, P>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
    C: Send + Sync + 'static,
    P: Send + Sync + 'static,
{
    let module = RpcModule::new(());
    Ok(module)
}

