// Integration tests for Encap/Decap algorithms
// These tests are gated behind the `with_curves` feature

#![cfg(feature = "with_curves")]

mod mimc_circuit;

use ark_bls12_381::{Bls12_381, Fr};
use ark_relations::gr1cs::ConstraintSynthesizer;
use ark_std::rand::{Rng, SeedableRng};
use mimc_circuit::{MiMCCircuit, MIMC_ROUNDS};
use zkenc_core::{decap, encap};

#[test]
fn test_encap_decap_correctness() {
    // Setup: Generate MiMC constants
    let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(0u64);
    let constants: Vec<Fr> = (0..MIMC_ROUNDS).map(|_| rng.gen()).collect();

    // Choose preimage (witness)
    let xl = Fr::from(42u64);
    let xr = Fr::from(99u64);
    let output = MiMCCircuit::mimc_native(xl, xr, &constants);

    // Public inputs: [output]
    let public_inputs = vec![output];

    // Witness: [xl, xr]
    let witness = vec![xl, xr];

    // Create circuit for Encap (with public inputs only)
    let circuit_encap = MiMCCircuit::new(None, None, Some(output), constants.clone());

    // Encap: Generate ciphertext and key
    let (ciphertext, key1) = encap::<Bls12_381, _, _>(circuit_encap, &mut rng).unwrap();

    // Create circuit for Decap (with full assignment)
    let circuit_decap = MiMCCircuit::new(Some(xl), Some(xr), Some(output), constants.clone());

    // Decap: Recover key using witness
    let key2 = decap::<Bls12_381, _>(circuit_decap, &ciphertext).unwrap();

    // Assert: Both keys should be identical
    assert_eq!(key1, key2, "Decap should recover the same key as Encap");

    println!("✅ Correctness test passed");
}

#[test]
fn test_encap_decap_wrong_witness() {
    let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(1u64);
    let constants: Vec<Fr> = (0..MIMC_ROUNDS).map(|_| rng.gen()).collect();

    // Correct preimage
    let xl = Fr::from(42u64);
    let xr = Fr::from(99u64);
    let output = MiMCCircuit::mimc_native(xl, xr, &constants);

    // Wrong witness (different preimage that doesn't satisfy the circuit)
    let wrong_xl = Fr::from(100u64);
    let wrong_xr = Fr::from(200u64);

    let circuit_encap = MiMCCircuit::new(None, None, Some(output), constants.clone());
    let (ciphertext, _key1) = encap::<Bls12_381, _, _>(circuit_encap, &mut rng).unwrap();

    let circuit_decap = MiMCCircuit::new(
        Some(wrong_xl),
        Some(wrong_xr),
        Some(output),
        constants.clone(),
    );

    // Decap with wrong witness should fail because circuit won't be satisfied
    let result = decap::<Bls12_381, _>(circuit_decap, &ciphertext);

    assert!(
        result.is_err(),
        "Decap should fail with wrong witness that doesn't satisfy circuit"
    );

    println!("✅ Wrong witness test passed: decap correctly rejected invalid witness");
}

#[test]
fn test_encap_different_public_inputs() {
    let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(2u64);
    let constants: Vec<Fr> = (0..MIMC_ROUNDS).map(|_| rng.gen()).collect();

    let xl = Fr::from(42u64);
    let xr = Fr::from(99u64);
    let output1 = MiMCCircuit::mimc_native(xl, xr, &constants);
    let output2 = Fr::from(999u64); // Different output

    let circuit1 = MiMCCircuit::new(None, None, Some(output1), constants.clone());
    let circuit2 = MiMCCircuit::new(None, None, Some(output2), constants.clone());

    let (ct1, key1) = encap::<Bls12_381, _, _>(circuit1, &mut rng).unwrap();
    let (ct2, key2) = encap::<Bls12_381, _, _>(circuit2, &mut rng).unwrap();

    // With different random parameters, CRS should be different
    // (alpha, beta, delta, r, x are sampled fresh each time)
    assert_ne!(
        ct1.encap_key.alpha_g1, ct2.encap_key.alpha_g1,
        "Different encap calls should have different CRS (different α)"
    );

    // Keys will also be different due to different randomness
    assert_ne!(
        key1, key2,
        "Different encap calls should produce different keys"
    );

    println!("✅ Different public inputs test passed");
}

#[test]
fn test_ciphertext_serialization() {
    let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(3u64);
    let constants: Vec<Fr> = (0..MIMC_ROUNDS).map(|_| rng.gen()).collect();

    let xl = Fr::from(1u64);
    let xr = Fr::from(2u64);
    let output = MiMCCircuit::mimc_native(xl, xr, &constants);

    let circuit = MiMCCircuit::new(None, None, Some(output), constants);

    let (ciphertext, _key) = encap::<Bls12_381, _, _>(circuit, &mut rng).unwrap();

    // Serialize and deserialize
    use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
    let mut bytes = Vec::new();
    ciphertext
        .serialize_compressed(&mut bytes)
        .expect("Serialization should succeed");
    let ciphertext2 = zkenc_core::Ciphertext::<Bls12_381>::deserialize_compressed(&bytes[..])
        .expect("Deserialization should succeed");

    assert_eq!(
        ciphertext.public_inputs, ciphertext2.public_inputs,
        "Public inputs should match after serialization"
    );

    println!("✅ Serialization test passed");
}

#[test]
fn test_mimc_circuit_integration() {
    // This test should pass immediately (doesn't require encap/decap)
    let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(4u64);
    let constants: Vec<Fr> = (0..MIMC_ROUNDS).map(|_| rng.gen()).collect();

    let xl = Fr::from(7u64);
    let xr = Fr::from(13u64);
    let output = MiMCCircuit::mimc_native(xl, xr, &constants);

    let circuit = MiMCCircuit::new(Some(xl), Some(xr), Some(output), constants);

    use ark_relations::gr1cs::{ConstraintSystem, OptimizationGoal};
    let cs = ConstraintSystem::new_ref();
    cs.set_optimization_goal(OptimizationGoal::Constraints);
    circuit.generate_constraints(cs.clone()).unwrap();

    assert!(
        cs.is_satisfied().unwrap(),
        "MiMC circuit should be satisfied with correct inputs"
    );
    println!("✅ MiMC integration test passed");
}
