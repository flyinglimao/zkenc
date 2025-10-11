//! zkenc-core
//!
//! WKEM (Witness Key Encapsulation Mechanism) for QAP

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(future_incompatible, nonstandard_style, rust_2018_idioms)]
#![allow(clippy::many_single_char_names, clippy::op_ref)]
#![forbid(unsafe_code)]

/// Data structures module
pub mod data_structures;

/// Core algorithms module
pub mod algorithm;

/// R1CS to QAP conversion utilities
mod r1cs_to_qap;

// Re-export commonly used types
pub use algorithm::{decap, encap, verify_ciphertext, Error};
pub use data_structures::{Ciphertext, EncapKey, Key};

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic() {
        // 基本測試
        assert!(true);
    }
}
