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

    // This will fail initially - we haven't implemented load_circom_circuit yet
    // Uncomment when ready to implement:
    // let (circuit, num_public) = zkenc_cli::circom::load_circom_circuit(&r1cs_path, &wasm_path).unwrap();
    // assert!(num_public > 0, "Should have public inputs");
}
