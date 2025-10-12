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

// Phase 3 Note: Full Circom integration test skipped due to arkworks version conflicts
// The integration requires matching versions of ark_ec, ark_ff, ark_relations between:
// - zkenc-core (uses git versions for 0.5)
// - ark_bls12_381 0.4 (uses crates.io 0.4)
//
// This will be resolved when:
// 1. zkenc-core is ported to use stable crates.io versions, OR
// 2. ark-circom is updated to support arkworks 0.5 git versions
//
// For now, zkenc-core's own tests demonstrate encap/decap work correctly.
// CLI integration will use:
// - parse_inputs() from Phase 2 ✅
// - CircomBuilder from ark-circom (when versions align)
// - zkenc-core's encap/decap (tested separately) ✅

#[test]
fn test_aes_gcm_roundtrip() {
    use zkenc_cli::crypto::{encrypt_gcm, decrypt_gcm};

    let key = b"01234567890123456789012345678901"; // Exactly 32 bytes
    let plaintext = b"Hello, zkenc! This is a secret message.";

    // Encrypt
    let result = encrypt_gcm(key, plaintext);
    assert!(result.is_ok(), "Encryption failed: {:?}", result.err());
    let ciphertext = result.unwrap();

    println!("AES-GCM Encryption:");
    println!("  Plaintext: {} bytes", plaintext.len());
    println!("  Ciphertext: {} bytes", ciphertext.len());

    // Decrypt
    let result = decrypt_gcm(key, &ciphertext);
    assert!(result.is_ok(), "Decryption failed: {:?}", result.err());
    let decrypted = result.unwrap();

    // Verify
    assert_eq!(plaintext.to_vec(), decrypted, "Decrypted text should match original");
    println!("  ✓ Roundtrip successful!");
}

#[test]
fn test_aes_ctr_roundtrip() {
    use zkenc_cli::crypto::{encrypt_ctr, decrypt_ctr};

    let key = b"01234567890123456789012345678901"; // Exactly 32 bytes
    let plaintext = b"Hello, zkenc! This is another secret message.";

    // Encrypt
    let result = encrypt_ctr(key, plaintext);
    assert!(result.is_ok(), "Encryption failed: {:?}", result.err());
    let ciphertext = result.unwrap();

    println!("AES-CTR Encryption:");
    println!("  Plaintext: {} bytes", plaintext.len());
    println!("  Ciphertext: {} bytes", ciphertext.len());

    // Decrypt
    let result = decrypt_ctr(key, &ciphertext);
    assert!(result.is_ok(), "Decryption failed: {:?}", result.err());
    let decrypted = result.unwrap();

    // Verify
    assert_eq!(plaintext.to_vec(), decrypted, "Decrypted text should match original");
    println!("  ✓ Roundtrip successful!");
}
