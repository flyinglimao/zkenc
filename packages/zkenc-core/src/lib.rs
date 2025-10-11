//! zkenc-core
//!
//! 核心演算法實作，支援 native 與 WASM 環境

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(future_incompatible, nonstandard_style, rust_2018_idioms)]
#![allow(clippy::many_single_char_names, clippy::op_ref)]
#![forbid(unsafe_code)]

/// 資料結構模組
pub mod data_structures;

/// 核心演算法模組
pub mod algorithm;

// 重新匯出常用型別
pub use algorithm::ZkEncAlgorithm;
pub use data_structures::{Proof, PublicParameters};

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic() {
        // 基本測試
        assert!(true);
    }
}
