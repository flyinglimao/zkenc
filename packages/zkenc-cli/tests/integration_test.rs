//! Integration tests for zkenc-cli circom support

use std::path::PathBuf;

#[test]
fn test_load_circom_circuit() {
    // Get test circuit paths
    let r1cs_path = PathBuf::from("tests/r1cs/signature.r1cs");
    let wasm_path = PathBuf::from("tests/r1cs/signature.wasm");

    // Verify files exist
    assert!(r1cs_path.exists(), "R1CS file should exist");
    assert!(wasm_path.exists(), "WASM file should exist");

    // Load circuit using circom module
    let result = zkenc_cli::circom::load_circom_circuit(&r1cs_path, &wasm_path);
    assert!(result.is_ok(), "Failed to load circuit: {:?}", result.err());

    let (num_constraints, num_public, num_variables) = result.unwrap();

    // Verify the circuit was loaded correctly
    assert!(num_constraints > 0, "Circuit should have constraints");
    assert!(num_public > 0, "Circuit should have public inputs");
    assert!(num_variables > 0, "Circuit should have variables");

    println!("Circuit info:");
    println!("  Constraints: {}", num_constraints);
    println!("  Public inputs: {}", num_public);
    println!("  Variables: {}", num_variables);
}

#[test]
fn test_parse_simple_input() {
    // Load a simple input file
    let input_path = PathBuf::from("tests/inputs/signature_basic.json");
    assert!(input_path.exists(), "Input file should exist");

    // Parse the input file
    let result = zkenc_cli::circom::parse_inputs(&input_path);
    assert!(result.is_ok(), "Failed to parse input: {:?}", result.err());

    let inputs = result.unwrap();

    // Verify we got some inputs
    assert!(!inputs.is_empty(), "Should have parsed some inputs");

    // Verify specific fields exist
    assert!(inputs.contains_key("message"), "Should have 'message' field");
    assert!(inputs.contains_key("publicKeys"), "Should have 'publicKeys' field");
    assert!(inputs.contains_key("R8"), "Should have 'R8' field");
    assert!(inputs.contains_key("S"), "Should have 'S' field");

    // Verify message is a single value
    let message = &inputs["message"];
    assert_eq!(message.len(), 1, "Message should be a single value");

    // Verify publicKeys is an array of arrays (3 keys × 2 coordinates)
    let public_keys = &inputs["publicKeys"];
    assert_eq!(public_keys.len(), 6, "publicKeys should have 6 elements (3 keys × 2 coords)");

    println!("Parsed {} input fields", inputs.len());
    for (key, values) in inputs.iter() {
        println!("  {}: {} values", key, values.len());
    }
}
