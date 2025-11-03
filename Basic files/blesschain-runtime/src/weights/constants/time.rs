//! Time-related constants used in BlessChain runtime.

/// Milliseconds per block (6 seconds = 6000ms).
pub const MILLISECS_PER_BLOCK: u64 = 6_000;

/// Slot duration for Aura (same as block time).
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;
