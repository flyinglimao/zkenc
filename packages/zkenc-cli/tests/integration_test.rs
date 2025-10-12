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
