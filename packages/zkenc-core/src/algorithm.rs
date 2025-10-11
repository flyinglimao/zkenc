//! WKEM (Witness Key Encapsulation Mechanism) for QAP
//!
//! Implementation of Encap, Decap, and Verify algorithms.

use ark_ec::pairing::Pairing;
use ark_ec::{CurveGroup, PrimeGroup, VariableBaseMSM};
use ark_ff::{Field, One, UniformRand};
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

    // Step 1: Synthesize circuit (may fail if witness is missing, but that's expected for encap)
    let cs = ConstraintSystem::new_ref();
    let synthesis_result = circuit.generate_constraints(cs.clone());

    // For encap without witness, synthesis may fail - that's OK
    // We only need the public inputs which should be assigned
    if let Err(e) = &synthesis_result {
        #[cfg(feature = "std")]
        println!(
            "  ⚠ Circuit synthesis error (expected for encap without witness): {:?}",
            e
        );
    }

    #[cfg(feature = "std")]
    println!(
        "  ✓ Circuit setup: {} constraints, {} variables",
        r1cs_to_qap::num_constraints(&cs),
        r1cs_to_qap::num_variables(&cs)
    );

    // Step 2: Extract public inputs from constraint system
    // Even if witness assignment failed, public inputs should be available
    let cs_borrowed = cs.borrow().unwrap();
    let instance_result = cs_borrowed.instance_assignment();
    let public_inputs: Vec<E::ScalarField> = match instance_result {
        Ok(assignment) => assignment[1..].to_vec(), // Skip the constant 1 at index 0
        Err(_) => {
            // If we can't get assignments, try to get number of public inputs at least
            #[cfg(feature = "std")]
            println!("  ⚠ Could not extract public input assignments");
            Vec::new()
        }
    };
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

    // Step 4: Create CRS σ by evaluating QAP polynomials
    let g1_generator = E::G1::generator();
    let g2_generator = E::G2::generator();

    #[cfg(feature = "std")]
    println!("  ⏳ Evaluating QAP polynomials at x...");

    // Evaluate u_i(x), v_i(x), w_i(x) for all variables
    let (u_evals, v_evals, w_evals) = r1cs_to_qap::evaluate_qap_polynomials_at_x(&cs, x);

    // Compute query vectors:
    // r_u_query_g1[i] = [r·u_i(x)]₁
    // r_v_query_g2[i] = [r·v_i(x)]₂
    // phi_delta_query_g1[i] = [φ_i(x)/δ]₁ where φ_i(x) = r·β·u_i(x) + r·α·v_i(x) + r²·w_i(x)
    let m = r1cs_to_qap::num_variables(&cs);
    let mut r_u_query_g1 = Vec::with_capacity(m);
    let mut r_v_query_g2 = Vec::with_capacity(m);
    let mut phi_delta_query_g1 = Vec::with_capacity(m);

    for i in 0..m {
        let r_u_i = r * u_evals[i];
        let r_v_i = r * v_evals[i];
        let phi_i = r * beta * u_evals[i] + r * alpha * v_evals[i] + r * r * w_evals[i];
        let phi_i_delta = phi_i * delta.inverse().expect("delta must be non-zero");

        r_u_query_g1.push((g1_generator * r_u_i).into_affine());
        r_v_query_g2.push((g2_generator * r_v_i).into_affine());
        phi_delta_query_g1.push((g1_generator * phi_i_delta).into_affine());
    }

    // h_query_g1: Placeholder for quotient polynomial evaluation
    // In full implementation: h(x) = (A(x)·B(x) - C(x)) / t(x) where t(x) is vanishing polynomial
    // For now, use empty vector as this is only needed for verification
    let h_query_g1 = Vec::new();

    #[cfg(feature = "std")]
    println!("  ✓ Generated CRS with {} query elements", m);

    let encap_key = EncapKey {
        alpha_g1: (g1_generator * alpha).into_affine(),
        beta_g2: (g2_generator * beta).into_affine(),
        delta_g2: (g2_generator * delta).into_affine(),
        r_u_query_g1,
        r_v_query_g2,
        phi_delta_query_g1,
        h_query_g1,
    };

    let ciphertext = Ciphertext {
        encap_key,
        public_inputs: public_inputs.clone(),
    };

    // Step 5: Compute pairing s = [α]₁ · [β]₂ (simplified for encap without witness)
    #[cfg(feature = "std")]
    println!("  ⏳ Computing pairing for key derivation...");

    // In encap, we only have public inputs, not full witness
    // For a full WKEM implementation with QAP evaluation, we would compute:
    // s = [α]₁ · [β]₂ + Σᵢ aᵢ · [φᵢ(x)]₁ · [1]₂
    // But since we don't have witness values, use simplified pairing with only public inputs

    // Compute Σᵢ aᵢ · [φᵢ(x)]₁ for public inputs only (indices 0 to l)
    let mut phi_sum_affine = Vec::new();
    let mut scalars = Vec::new();

    // Add constant 1 at index 0
    if !ciphertext.encap_key.phi_delta_query_g1.is_empty() {
        phi_sum_affine.push(ciphertext.encap_key.phi_delta_query_g1[0]);
        scalars.push(E::ScalarField::one());
    }

    // Add public inputs
    for (idx, &a_i) in public_inputs.iter().enumerate() {
        let i = idx + 1; // Skip constant 1 at index 0
        if i < ciphertext.encap_key.phi_delta_query_g1.len() {
            phi_sum_affine.push(ciphertext.encap_key.phi_delta_query_g1[i]);
            scalars.push(a_i);
        }
    }

    // MSM: compute Σᵢ aᵢ · Pᵢ for public inputs
    let phi_sum = if !phi_sum_affine.is_empty() {
        E::G1::msm(&phi_sum_affine, &scalars).map_err(|_| Error::SerializationError)?
    } else {
        // If no public inputs, use zero (identity in additive group)
        g1_generator - g1_generator
    };

    // Compute s = [α]₁ · [β]₂ + (Σᵢ aᵢ · [φᵢ(x)]₁) · [1]₂
    let pairing1 = E::pairing(ciphertext.encap_key.alpha_g1, ciphertext.encap_key.beta_g2);
    let pairing2 = E::pairing(phi_sum, g2_generator);
    let s = pairing1 + pairing2;

    // Serialize pairing result and hash to get key
    use ark_serialize::CanonicalSerialize;
    let mut s_bytes = Vec::new();
    s.serialize_compressed(&mut s_bytes)
        .map_err(|_| Error::SerializationError)?;

    // Derive the key from pairing result
    // In production, should use proper KDF like HKDF or Blake3
    // For now, use first 32 bytes of serialized pairing result
    let mut key_bytes = [0u8; 32];
    let len = core::cmp::min(32, s_bytes.len());
    key_bytes[..len].copy_from_slice(&s_bytes[..len]);

    let key = Key::new(key_bytes);

    #[cfg(feature = "std")]
    println!("  ✓ Derived key from pairing");

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
