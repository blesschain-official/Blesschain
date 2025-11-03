#![cfg_attr(not(test), no_std)]

#[cfg(not(target_arch = "wasm32"))]
mod sys;

#[cfg(not(target_arch = "wasm32"))]
pub use sys::*;

#[cfg(target_arch = "wasm32")]
pub fn errno() -> i32 {
    0
}

