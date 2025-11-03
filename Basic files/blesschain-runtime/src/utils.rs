// blesschain-runtime/src/utils.rs

//! Helper functions for development and testing.

use sp_core::sr25519; use sp_core::Pair; use sp_runtime::AccountId32;

/// Generate an AccountId from a seed string (e.g. "//Alice"). pub fn get_account_id_from_seed(name: &str) -> AccountId32 { let pair = sr25519::Pair::from_string(&format!("//{}", name), None) .expect("Static values are valid; qed"); pair.public().into() }








