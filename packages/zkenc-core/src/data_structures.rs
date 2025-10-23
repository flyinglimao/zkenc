//! Data structures for WKEM (Witness Key Encapsulation Mechanism) for QAP
//!
//! This module defines the core data structures used in the WKEM scheme:
//! - EncapKey: The Common Reference String (CRS) σ generated during Encap
//! - Ciphertext: Contains EncapKey and public inputs
//! - Key: The derived symmetric key (32 bytes from Blake3)

use ark_ec::pairing::Pairing;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::vec::Vec;

/// Common Reference String (CRS) σ
///
/// Contains all group elements needed for Decap to recover the key.
/// Generated fresh for each Encap operation.
#[derive(Clone, Debug, PartialEq, CanonicalSerialize, CanonicalDeserialize)]
pub struct EncapKey<E: Pairing> {
    /// [α]₁ - randomness component in G1
    pub alpha_g1: E::G1Affine,

    /// [β]₂ - randomness component in G2
    pub beta_g2: E::G2Affine,

    /// [δ]₂ - denominator for witness part
    pub delta_g2: E::G2Affine,

    /// {[r·uᵢ(x)]₁}ᵢ₌₀^m - scaled U polynomial evaluations in G1
    pub r_u_query_g1: Vec<E::G1Affine>,

    /// {[r·vᵢ(x)]₂}ᵢ₌₀^m - scaled V polynomial evaluations in G2
    pub r_v_query_g2: Vec<E::G2Affine>,

    /// {[φᵢ(x)/δ]₁}ᵢ₌ℓ₊₁^m - combined polynomial for witness variables
    /// where φᵢ(x) = r·β·uᵢ(x) + r·α·vᵢ(x) + r²·wᵢ(x)
    pub phi_delta_query_g1: Vec<E::G1Affine>,

    /// {[r²·xⁱ·t(x)/δ]₁}ᵢ₌₀^(n-2) - for computing quotient polynomial h(x)
    pub h_query_g1: Vec<E::G1Affine>,
}

/// Ciphertext containing CRS and public inputs
#[derive(Clone, Debug, PartialEq, CanonicalSerialize, CanonicalDeserialize)]
pub struct Ciphertext<E: Pairing> {
    /// The encapsulation key (CRS σ)
    pub encap_key: EncapKey<E>,

    /// Public inputs {aᵢ}ᵢ₌₀^ℓ
    pub public_inputs: Vec<E::ScalarField>,
}

/// Derived symmetric key (output of Blake3 hash)
#[derive(Clone, Debug, PartialEq, Eq, CanonicalSerialize, CanonicalDeserialize)]
pub struct Key(pub [u8; 32]);

impl Key {
    /// Create a new key from 32 bytes
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Get the key bytes
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}
