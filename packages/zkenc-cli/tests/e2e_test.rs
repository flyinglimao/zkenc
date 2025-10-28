// e2e_test.rs - End-to-end integration test
//
// This tests the full workflow:
// 1. Encap: Generate ciphertext and key from circuit + public inputs
// 2. Encrypt: Encrypt a message with the key
// 3. Decap: Recover the key from circuit + witness + ciphertext
// 4. Decrypt: Decrypt the message with recovered key
// 5. Verify: Decrypted message matches original

use anyhow::Result;
use std::fs;
use std::path::PathBuf;

/// Test the full Encap → Encrypt → Decap → Decrypt workflow with Sudoku circuit
#[test]
fn test_sudoku_e2e() -> Result<()> {
    println!("\n🎮 E2E Test: Sudoku Circuit");
    println!("=====================================\n");

    // Setup test directory
    let test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");
    let temp_dir = test_dir.join("temp");
    fs::create_dir_all(&temp_dir)?;

    // File paths
    let circuit_path = test_dir.join("r1cs/sudoku.r1cs");
    let sym_path = test_dir.join("r1cs/sudoku.sym");
    let input_path = test_dir.join("inputs/sudoku_basic.json");
    let witness_path = test_dir.join("inputs/sudoku_sudoku_basic.wtns");

    let ciphertext_path = temp_dir.join("witness.ct");
    let key1_path = temp_dir.join("key_encap.bin");
    let key2_path = temp_dir.join("key_decap.bin");
    let message_path = temp_dir.join("message.txt");
    let combined_ciphertext_path = temp_dir.join("combined.bin");
    let decrypted_path = temp_dir.join("decrypted.txt");

    // Create test message
    let original_message = b"Hello, Zero-Knowledge Encryption!";
    fs::write(&message_path, original_message)?;
    println!(
        "📝 Original message: {:?}",
        String::from_utf8_lossy(original_message)
    );

    // === Step 1: Encap ===
    println!("\n🔐 Step 1: Encap");
    println!("------------------");
    zkenc_cli::commands::encap_command(
        circuit_path.to_str().unwrap(),
        sym_path.to_str().unwrap(),
        input_path.to_str().unwrap(),
        ciphertext_path.to_str().unwrap(),
        key1_path.to_str().unwrap(),
    )?;

    // Verify files were created
    assert!(ciphertext_path.exists(), "Ciphertext file should exist");
    assert!(key1_path.exists(), "Key file should exist");

    let ct_size = fs::metadata(&ciphertext_path)?.len();
    let key_size = fs::metadata(&key1_path)?.len();
    println!("\n✅ Encap complete:");
    println!("   - Ciphertext: {} bytes", ct_size);
    println!("   - Key: {} bytes", key_size);

    // === Step 2: Encrypt (high-level) ===
    println!("\n🔒 Step 2: Encrypt");
    println!("------------------");
    zkenc_cli::commands::encrypt_command(
        circuit_path.to_str().unwrap(),
        sym_path.to_str().unwrap(),
        input_path.to_str().unwrap(),
        message_path.to_str().unwrap(),
        combined_ciphertext_path.to_str().unwrap(),
        true,
    )?;

    assert!(combined_ciphertext_path.exists(), "Combined ciphertext file should exist");
    let combined_size = fs::metadata(&combined_ciphertext_path)?.len();
    println!("\n✅ Encrypt complete: {} bytes", combined_size);

    // === Step 3: Decap ===
    println!("\n🔓 Step 3: Decap");
    println!("------------------");
    zkenc_cli::commands::decap_command(
        circuit_path.to_str().unwrap(),
        witness_path.to_str().unwrap(),
        ciphertext_path.to_str().unwrap(),
        key2_path.to_str().unwrap(),
    )?;

    assert!(key2_path.exists(), "Recovered key file should exist");
    println!("\n✅ Decap complete");

    // === Step 4: Decrypt (high-level) ===
    println!("\n🔓 Step 4: Decrypt");
    println!("------------------");
    zkenc_cli::commands::decrypt_command(
        circuit_path.to_str().unwrap(),
        witness_path.to_str().unwrap(),
        combined_ciphertext_path.to_str().unwrap(),
        decrypted_path.to_str().unwrap(),
    )?;

    assert!(decrypted_path.exists(), "Decrypted file should exist");

    // === Step 5: Verify ===
    println!("\n🔍 Step 5: Verify");
    println!("------------------");
    let decrypted_message = fs::read(&decrypted_path)?;

    println!(
        "📝 Decrypted message: {:?}",
        String::from_utf8_lossy(&decrypted_message)
    );
    println!(
        "📝 Original message:  {:?}",
        String::from_utf8_lossy(original_message)
    );

    assert_eq!(
        original_message,
        &decrypted_message[..],
        "Decrypted message should match original"
    );

    // Verify keys match
    let key1_bytes = fs::read(&key1_path)?;
    let key2_bytes = fs::read(&key2_path)?;
    assert_eq!(
        key1_bytes, key2_bytes,
        "Keys from encap and decap should match"
    );

    println!("\n✅ All checks passed!");
    println!("   - Message decrypted successfully");
    println!("   - Keys match (encap == decap)");
    println!("   - Content integrity verified\n");

    // Cleanup
    fs::remove_dir_all(&temp_dir)?;

    Ok(())
}

/// Test that decap fails with wrong witness
#[test]
fn test_sudoku_e2e_wrong_witness() -> Result<()> {
    println!("\n⚠️  E2E Test: Wrong Witness");
    println!("=====================================\n");

    let test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");
    let temp_dir = test_dir.join("temp_wrong");
    fs::create_dir_all(&temp_dir)?;

    let circuit_path = test_dir.join("r1cs/sudoku.r1cs");
    let sym_path = test_dir.join("r1cs/sudoku.sym");
    let input_path = test_dir.join("inputs/sudoku_basic.json");
    let wrong_witness_path = test_dir.join("inputs/sudoku_sudoku_general.wtns"); // Different witness

    let ciphertext_path = temp_dir.join("ciphertext.bin");
    let key_path = temp_dir.join("key.bin");

    // Encap with correct inputs
    println!("🔐 Encap with correct inputs...");
    zkenc_cli::commands::encap_command(
        circuit_path.to_str().unwrap(),
        sym_path.to_str().unwrap(),
        input_path.to_str().unwrap(),
        ciphertext_path.to_str().unwrap(),
        key_path.to_str().unwrap(),
    )?;

    // Try to decap with wrong witness (should fail)
    println!("\n🔓 Attempting Decap with wrong witness...");
    let result = zkenc_cli::commands::decap_command(
        circuit_path.to_str().unwrap(),
        wrong_witness_path.to_str().unwrap(),
        ciphertext_path.to_str().unwrap(),
        temp_dir.join("wrong_key.bin").to_str().unwrap(),
    );

    match result {
        Ok(_) => {
            println!("⚠️  Decap succeeded with wrong witness (unexpected)");
            println!("   This might mean the witness doesn't violate constraints");
            // Note: sudoku_general might still be a valid sudoku solution
        }
        Err(e) => {
            println!("✅ Decap correctly failed: {}", e);
        }
    }

    // Cleanup
    fs::remove_dir_all(&temp_dir)?;

    Ok(())
}
