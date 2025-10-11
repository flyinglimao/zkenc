//! 資料結構定義
//!
//! 定義核心演算法使用的資料結構

use ark_ec::pairing::Pairing;
use ark_serialize::*;
use ark_std::vec::Vec;

/// 範例：證明結構
#[derive(Clone, Debug, PartialEq, CanonicalSerialize, CanonicalDeserialize)]
pub struct Proof<E: Pairing> {
    /// 證明元素
    pub elements: Vec<E::G1Affine>,
}

impl<E: Pairing> Default for Proof<E> {
    fn default() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
}

/// 範例：公開參數
#[derive(Clone, Debug, PartialEq, CanonicalSerialize, CanonicalDeserialize)]
pub struct PublicParameters<E: Pairing> {
    /// G1 生成元
    pub g1_generator: E::G1Affine,
    /// G2 生成元
    pub g2_generator: E::G2Affine,
}

impl<E: Pairing> Default for PublicParameters<E> {
    fn default() -> Self {
        Self {
            g1_generator: E::G1Affine::default(),
            g2_generator: E::G2Affine::default(),
        }
    }
}
