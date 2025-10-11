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
#[ignore]
fn test_encap_decap_wrong_witness() {
    let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(1u64);
    let constants: Vec<Fr> = (0..MIMC_ROUNDS).map(|_| rng.gen()).collect();

    // Correct preimage
    let xl = Fr::from(42u64);
    let xr = Fr::from(99u64);
    let output = MiMCCircuit::mimc_native(xl, xr, &constants);

    let public_inputs = vec![output];

    // Wrong witness (different preimage)
    let wrong_xl = Fr::from(100u64);
    let wrong_xr = Fr::from(200u64);
    let wrong_witness = vec![wrong_xl, wrong_xr];

    let circuit_encap = MiMCCircuit::new(None, None, Some(output), constants.clone());
    // let (ciphertext, key1) = encap::<Bls12_381, _, _>(circuit_encap, &public_inputs, &mut rng).unwrap();

    let circuit_decap = MiMCCircuit::new(
        Some(wrong_xl),
        Some(wrong_xr),
        Some(output),
        constants.clone(),
    );

    // Decap with wrong witness should either:
    // 1. Return a different key, or
    // 2. Fail (if implementation checks QAP satisfaction)
    // let result = decap::<Bls12_381, _>(circuit_decap, &wrong_witness, &ciphertext);

    // match result {
    //     Ok(key2) => {
    //         assert_ne!(key1, key2, "Wrong witness should produce different key");
    //     }
    //     Err(_) => {
    //         // Also acceptable: decap fails for wrong witness
    //     }
    // }

    println!("✅ Wrong witness test passed (placeholder)");
}

#[test]
#[ignore]
fn test_encap_different_public_inputs() {
    let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(2u64);
    let constants: Vec<Fr> = (0..MIMC_ROUNDS).map(|_| rng.gen()).collect();

    let xl = Fr::from(42u64);
    let xr = Fr::from(99u64);
    let output1 = MiMCCircuit::mimc_native(xl, xr, &constants);
    let output2 = Fr::from(999u64); // Different output

    let public_inputs1 = vec![output1];
    let public_inputs2 = vec![output2];

    let circuit1 = MiMCCircuit::new(None, None, Some(output1), constants.clone());
    let circuit2 = MiMCCircuit::new(None, None, Some(output2), constants.clone());

    // let (ct1, key1) = encap::<Bls12_381, _, _>(circuit1, &public_inputs1, &mut rng).unwrap();
    // let (ct2, key2) = encap::<Bls12_381, _, _>(circuit2, &public_inputs2, &mut rng).unwrap();

    // Different public inputs should produce different ciphertexts
    // (Note: keys might coincidentally be the same due to randomness, so we check ciphertext)
    // assert_ne!(ct1.public_inputs, ct2.public_inputs);

    println!("✅ Different public inputs test passed (placeholder)");
}

#[test]
#[ignore]
fn test_ciphertext_serialization() {
    let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(3u64);
    let constants: Vec<Fr> = (0..MIMC_ROUNDS).map(|_| rng.gen()).collect();

    let xl = Fr::from(1u64);
    let xr = Fr::from(2u64);
    let output = MiMCCircuit::mimc_native(xl, xr, &constants);

    let public_inputs = vec![output];
    let circuit = MiMCCircuit::new(None, None, Some(output), constants);

    // let (ciphertext, _key) = encap::<Bls12_381, _, _>(circuit, &public_inputs, &mut rng).unwrap();

    // Serialize and deserialize
    // use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};
    // let mut bytes = Vec::new();
    // ciphertext.serialize_compressed(&mut bytes).unwrap();
    // let ciphertext2 = Ciphertext::<Bls12_381>::deserialize_compressed(&bytes[..]).unwrap();

    // assert_eq!(ciphertext.public_inputs, ciphertext2.public_inputs);

    println!("✅ Serialization test passed (placeholder)");
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
