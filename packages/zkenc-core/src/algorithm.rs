//! WKEM (Witness Key Encapsulation Mechanism) for QAP
//!
//! Implementation of Encap, Decap, and Verify algorithms.

use ark_ec::pairing::Pairing;
use ark_ec::{CurveGroup, PrimeGroup};
use ark_ff::UniformRand;
use ark_relations::gr1cs::{ConstraintSynthesizer, ConstraintSystem};
use ark_std::rand::RngCore;
use ark_std::vec::Vec;

use crate::data_structures::{Ciphertext, EncapKey, Key};
use crate::r1cs_to_qap;

/// Error types for WKEM operations
#[derive(Debug)]
pub enum Error {
    /// Circuit synthesis failed
    SynthesisError(String),
    /// Invalid witness (QAP not satisfied)
    InvalidWitness,
    /// Invalid public inputs
    InvalidPublicInputs,
    /// Serialization error
    SerializationError,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::SynthesisError(msg) => write!(f, "Synthesis error: {}", msg),
            Error::InvalidWitness => write!(f, "Invalid witness: QAP relation not satisfied"),
            Error::InvalidPublicInputs => write!(f, "Invalid public inputs"),
            Error::SerializationError => write!(f, "Serialization error"),
        }
    }
}

/// Encapsulate: Generate ciphertext and key from circuit with public inputs only
///
/// # Arguments
/// * `circuit` - Circuit with public inputs assigned, witness unassigned
/// * `rng` - Random number generator
///
/// # Returns
/// * `(Ciphertext, Key)` - The ciphertext containing CRS σ and the derived key
pub fn encap<E, C, R>(circuit: C, rng: &mut R) -> Result<(Ciphertext<E>, Key), Error>
where
    E: Pairing,
    C: ConstraintSynthesizer<E::ScalarField>,
    R: RngCore,
{
    #[cfg(feature = "std")]
    println!("🔐 Starting Encap...");

    // Step 1: Synthesize circuit
    let cs = ConstraintSystem::new_ref();
    circuit
        .generate_constraints(cs.clone())
        .map_err(|e| Error::SynthesisError(format!("{:?}", e)))?;

    #[cfg(feature = "std")]
    println!(
        "  ✓ Circuit synthesized: {} constraints, {} variables",
        r1cs_to_qap::num_constraints(&cs),
        r1cs_to_qap::num_variables(&cs)
    );

    // Step 2: Extract public inputs from constraint system
    let cs_borrowed = cs.borrow().unwrap();
    let instance_assignment = cs_borrowed
        .instance_assignment()
        .map_err(|e| Error::SynthesisError(format!("{:?}", e)))?;
    let public_inputs: Vec<E::ScalarField> = instance_assignment[1..].to_vec(); // Skip the constant 1 at index 0
    drop(cs_borrowed); // Release the borrow

    #[cfg(feature = "std")]
    println!("  ✓ Extracted {} public inputs", public_inputs.len());

    // Step 3: Sample random parameters
    let alpha = E::ScalarField::rand(rng);
    let beta = E::ScalarField::rand(rng);
    let delta = E::ScalarField::rand(rng);
    let r = E::ScalarField::rand(rng);
    let x = E::ScalarField::rand(rng);

    #[cfg(feature = "std")]
    println!("  ✓ Sampled random parameters (α, β, δ, r, x)");

    // Step 4: Create placeholder CRS (TODO: implement polynomial evaluations)
    let g1_generator = E::G1::generator();
    let g2_generator = E::G2::generator();

    let encap_key = EncapKey {
        alpha_g1: (g1_generator * alpha).into_affine(),
        beta_g2: (g2_generator * beta).into_affine(),
        delta_g2: (g2_generator * delta).into_affine(),
        r_u_query_g1: Vec::new(),       // TODO
        r_v_query_g2: Vec::new(),       // TODO
        phi_delta_query_g1: Vec::new(), // TODO
        h_query_g1: Vec::new(),         // TODO
    };

    let ciphertext = Ciphertext {
        encap_key,
        public_inputs,
    };

    // Step 5: Compute placeholder key (TODO: implement pairing and Keccak256)
    let key = Key::new([0u8; 32]); // TODO

    #[cfg(feature = "std")]
    println!("  ✓ Generated CRS and derived key");

    Ok((ciphertext, key))
}

/// Decapsulate: Recover key using witness
///
/// # Arguments
/// * `circuit` - Circuit with full assignment (public inputs + witness)
/// * `ciphertext` - Ciphertext from Encap containing CRS σ
///
/// # Returns
/// * `Key` - The recovered symmetric key
pub fn decap<E, C>(circuit: C, ciphertext: &Ciphertext<E>) -> Result<Key, Error>
where
    E: Pairing,
    C: ConstraintSynthesizer<E::ScalarField>,
{
    #[cfg(feature = "std")]
    println!("🔓 Starting Decap...");

    // Step 1: Synthesize circuit with witness
    let cs = ConstraintSystem::new_ref();
    circuit
        .generate_constraints(cs.clone())
        .map_err(|e| Error::SynthesisError(format!("{:?}", e)))?;

    // Verify constraint system is satisfied
    if !cs.is_satisfied().unwrap_or(false) {
        #[cfg(feature = "std")]
        eprintln!("  ✗ Circuit constraints not satisfied!");
        return Err(Error::InvalidWitness);
    }

    #[cfg(feature = "std")]
    println!("  ✓ Circuit synthesized and satisfied");

    // TODO: Implement key recovery
    let _ = ciphertext;
    todo!("Decap implementation in progress")
}

/// Verify that a ciphertext is well-formed
///
/// # Arguments
/// * `ciphertext` - Ciphertext to verify
/// * `expected_public_inputs` - Expected public inputs
///
/// # Returns
/// * `bool` - True if verification passes
pub fn verify_ciphertext<E>(
    ciphertext: &Ciphertext<E>,
    expected_public_inputs: &[E::ScalarField],
) -> bool
where
    E: Pairing,
{
    #[cfg(feature = "std")]
    println!("🔍 Verifying ciphertext...");

    if ciphertext.public_inputs.len() != expected_public_inputs.len() {
        #[cfg(feature = "std")]
        eprintln!("  ✗ Public input length mismatch");
        return false;
    }

    for (a, b) in ciphertext
        .public_inputs
        .iter()
        .zip(expected_public_inputs.iter())
    {
        if a != b {
            #[cfg(feature = "std")]
            eprintln!("  ✗ Public input mismatch");
            return false;
        }
    }

    #[cfg(feature = "std")]
    println!("  ✓ Public inputs verified");
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::InvalidWitness;
        assert!(format!("{}", err).contains("Invalid witness"));
    }
}
