//! Time and block duration constants for BlessChain Runtime.

#![allow(dead_code)]

/// Milliseconds per block. Default was 2000 (2 seconds), now set to 7000 (7 seconds).
pub const MILLISECS_PER_BLOCK: u64 = 7000;

/// Slot duration — Aura uses this for consensus timing.
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

/// Seconds per block (for reference or other pallet time calculations).
pub const SECS_PER_BLOCK: u64 = MILLISECS_PER_BLOCK / 1000;

/// Number of blocks per minute (approximation).
pub const MINUTES: u64 = 60 / SECS_PER_BLOCK;

