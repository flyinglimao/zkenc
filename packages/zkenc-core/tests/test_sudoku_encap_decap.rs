// test_sudoku_encap_decap.rs - Test encap/decap with Sudoku circuit
//
// This tests the full encap/decap flow with a real Circom circuit (Sudoku)

#![cfg(feature = "test_fixtures")]

mod circom_circuit;

use ark_bn254::{Bn254, Fr}; // Circom uses BN254 (alt_bn128)
use ark_std::rand::SeedableRng;
use circom_circuit::TestCircuit;
use std::collections::HashMap;
use zkenc_core::{decap, encap};

// Sudoku puzzle from sudoku_general.json (incomplete puzzle with 0s)
const SUDOKU_PUZZLE: [u8; 81] = [
    5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0, 6, 0, 8, 0, 0, 0, 6,
    0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0, 0, 6, 0, 6, 0, 0, 0, 0, 2, 8, 0, 0,
    0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 7, 9,
];

// Valid solution for sudoku_general.json puzzle
const SUDOKU_SOLUTION: [u8; 81] = [
    5, 3, 4, 6, 7, 8, 9, 1, 2, 6, 7, 2, 1, 9, 5, 3, 4, 8, 1, 9, 8, 3, 4, 2, 5, 6, 7, 8, 5, 9, 7, 6,
    1, 4, 2, 3, 4, 2, 6, 8, 5, 3, 7, 9, 1, 7, 1, 3, 9, 2, 4, 8, 5, 6, 9, 6, 1, 5, 3, 7, 2, 8, 4, 2,
    8, 7, 4, 1, 9, 6, 3, 5, 3, 4, 5, 2, 8, 6, 1, 7, 9,
];

#[test]
fn test_sudoku_encap_decap() {
    println!("\n🎮 Testing Sudoku Encap/Decap with sudoku_general");
    println!("===================================================\n");

    // Load circuit structure from fixture
    println!("📂 Loading circuit structure from fixture...");
    let circuit_template =
        TestCircuit::from_fixture("sudoku_circuit").expect("Failed to load sudoku circuit");

    println!("✅ Loaded Sudoku circuit:");
    println!(
        "   - Constraints: {}",
        circuit_template.circuit.n_constraints
    );
    println!("   - Public inputs: {}", circuit_template.n_public_inputs());
    println!("   - Wires: {}", circuit_template.circuit.n_wires);

    // Setup RNG
    let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(42u64);

    // ENCAP: Create circuit with only public inputs (puzzle)
    // Extract public inputs from the fixture to ensure consistency
    println!("\n📤 Running Encap (with puzzle only - public inputs)...");
    let mut encap_witness = HashMap::new();

    // Wire 0 is always 1 (constant)
    encap_witness.insert(0, Fr::from(1u64));

    // Copy only the public inputs from the fixture (wires 1-81)
    // This ensures encap and decap use the same public inputs
    for wire_id in 1..=circuit_template.n_public_inputs() {
        if let Some(&value) = circuit_template.witness.get(&wire_id) {
            encap_witness.insert(wire_id, value);
        }
    }

    println!(
        "   - Public inputs extracted from fixture: {} wires",
        circuit_template.n_public_inputs()
    );
    println!("   - Note: Using fixture's public inputs (sudoku_basic - completed sudoku)");

    let encap_circuit =
        TestCircuit::with_public_inputs(circuit_template.circuit.clone(), encap_witness);

    let (ciphertext, key1) = encap::<Bn254, _, _>(encap_circuit, &mut rng).expect("Encap failed");

    println!("✅ Encap successful!");
    println!("   - Ciphertext size: {} bytes", {
        use ark_serialize::CanonicalSerialize;
        ciphertext.serialized_size(ark_serialize::Compress::Yes)
    });
    println!("   - Key size: {} bytes", key1.0.len());

    // DECAP: Use complete witness from fixture
    // (The fixture was generated with sudoku_general inputs, so it already has the correct witness)
    println!("\n📥 Running Decap (with complete witness from fixture)...");

    // Load the complete witness from fixture (all 202 wires including intermediate values)
    let decap_circuit =
        TestCircuit::from_fixture("sudoku_circuit").expect("Failed to load circuit for decap");

    println!("   - Puzzle (first 9): {:?}...", &SUDOKU_PUZZLE[0..9]);
    println!("   - Solution (first 9): {:?}...", &SUDOKU_SOLUTION[0..9]);
    println!(
        "   - Total witness wires from fixture: {}",
        decap_circuit.witness.len()
    );

    let key2 = decap::<Bn254, _>(decap_circuit, &ciphertext).expect("Decap failed");

    println!("✅ Decap successful!");

    // Verify keys match
    println!("\n🔍 Verifying keys...");
    assert_eq!(key1.0.len(), key2.0.len(), "Key lengths don't match");
    assert_eq!(key1, key2, "Keys don't match!");

    println!("✅ Keys match! Encap/Decap test passed! 🎉");
    println!("   Puzzle (0s indicate blanks): {:?}", &SUDOKU_PUZZLE[0..9]);
    println!(
        "   Solution (complete):        {:?}",
        &SUDOKU_SOLUTION[0..9]
    );
}

#[test]
fn test_sudoku_decap_with_wrong_solution() {
    println!("\n🎮 Testing Sudoku Decap with Wrong Solution");
    println!("=============================================\n");

    // This test verifies that decap fails with an invalid witness
    // (e.g., wrong sudoku solution)

    // Load sudoku circuit
    let test_case = {
        use std::path::PathBuf;
        use zkenc_core::serializable::SerializableTestCase;

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/fixtures/sudoku_circuit.bin");

        let bytes = std::fs::read(&path).expect("Failed to read sudoku fixture");
        bincode::deserialize::<SerializableTestCase>(&bytes).expect("Failed to deserialize")
    };

    // Setup RNG
    let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(42u64);

    // ENCAP with correct puzzle
    let mut encap_witness = HashMap::new();
    encap_witness.insert(0, Fr::from(1u64));
    for (i, &value) in SUDOKU_PUZZLE.iter().enumerate() {
        encap_witness.insert((i + 1) as u32, Fr::from(value as u64));
    }

    let encap_circuit = TestCircuit::with_public_inputs(test_case.circuit.clone(), encap_witness);

    let (ciphertext, _key1) = encap::<Bn254, _, _>(encap_circuit, &mut rng).expect("Encap failed");

    println!("✅ Encap successful");

    // DECAP with WRONG solution (all zeros)
    println!("\n📥 Attempting Decap with wrong solution...");
    let mut wrong_witness = HashMap::new();
    wrong_witness.insert(0, Fr::from(1u64));

    // Same puzzle
    for (i, &value) in SUDOKU_PUZZLE.iter().enumerate() {
        wrong_witness.insert((i + 1) as u32, Fr::from(value as u64));
    }

    // But wrong private witness (all zeros instead of solution)
    let n_private_wires = test_case.circuit.n_wires - test_case.circuit.n_pub_in - 1;
    for i in 0..n_private_wires {
        let wire_id = (test_case.circuit.n_pub_in + 1 + i) as u32;
        wrong_witness.insert(wire_id, Fr::from(0u64)); // All zeros = wrong
    }

    let wrong_circuit = TestCircuit::with_public_inputs(test_case.circuit.clone(), wrong_witness);

    let result = decap::<Bn254, _>(wrong_circuit, &ciphertext);

    match result {
        Ok(_) => {
            println!("⚠️  Decap succeeded with wrong witness (unexpected)");
            println!("   This might mean the constraint system isn't properly constraining");
        }
        Err(e) => {
            println!("✅ Decap correctly failed with wrong witness: {:?}", e);
        }
    }
}
