//! WASM bindings for zkenc-core
//!
//! This module provides WebAssembly bindings for the core witness encryption functionality.

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
use ark_bn254::{Bn254, Fr};
#[cfg(feature = "wasm")]
use ark_ff::PrimeField;
#[cfg(feature = "wasm")]
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
#[cfg(feature = "wasm")]
use ark_std::rand::rngs::StdRng;
#[cfg(feature = "wasm")]
use ark_std::rand::SeedableRng;

#[cfg(feature = "wasm")]
use crate::{decap, encap, Ciphertext};

/// Initialize WASM module (sets up panic hook for better error messages)
#[cfg(feature = "wasm")]
#[wasm_bindgen(start)]
pub fn wasm_init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Encapsulation result containing ciphertext and key
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct WasmEncapResult {
    ciphertext: Vec<u8>,
    key: Vec<u8>,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl WasmEncapResult {
    /// Get ciphertext as bytes
    #[wasm_bindgen(getter)]
    pub fn ciphertext(&self) -> Vec<u8> {
        self.ciphertext.clone()
    }

    /// Get key as bytes (32 bytes)
    #[wasm_bindgen(getter)]
    pub fn key(&self) -> Vec<u8> {
        self.key.clone()
    }
}

// Note: The actual WASM bindings will be completed once we have:
// 1. A way to parse R1CS from bytes (currently in zkenc-cli)
// 2. A way to parse witness from bytes (currently in zkenc-cli)
// 3. A Circuit implementation that works with parsed data
//
// For now, zkenc-js will handle these in its own Rust code until
// we refactor the parsers into zkenc-core.
