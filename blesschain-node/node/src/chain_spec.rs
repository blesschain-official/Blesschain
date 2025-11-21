// This file is part of Substrate.
//
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

//! Minimal ChainSpec for BlessChain.
//! For now we do NOT build a dev spec in code. You can use a JSON file with
//! `--chain <path>` if needed.

use sc_service::GenericChainSpec;

/// Extension type for ChainSpec.
/// `Option<()>` implements `ChainSpecExtension`, so this satisfies the trait
/// bounds required by `GenericChainSpec`.
pub type ChainSpec = GenericChainSpec<Option<()>>;

/// Placeholder development chain spec.
///
/// Currently not implemented. If you pass `--chain dev`, this will return
/// an error. Use a JSON chainspec file instead.
pub fn development_chain_spec() -> Result<ChainSpec, String> {
    Err("development_chain_spec is not implemented for BlessChain minimal node. \
Use a JSON file with `--chain <path-to-chainspec.json>` instead."
        .into())
}

