// commands.rs - CLI command implementations
//
// This module implements the actual logic for CLI commands:
// - encap: Generate ciphertext and key from circuit + public inputs
// - decap: Recover key from circuit + witness + ciphertext
// - encrypt: High-level encryption (encap + AES, compatible with zkenc-js)
// - decrypt: High-level decryption (decap + AES, compatible with zkenc-js)

use anyhow::{Context, Result};
use ark_bn254::{Bn254, Fr};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use std::collections::HashMap;
use std::fs;

use crate::circuit::CircomCircuit;
use crate::crypto;
use crate::r1cs::R1csFile;
use crate::witness::WitnessFile;
use zkenc_core::{decap, encap, Ciphertext, Key};

/// Encap command: Generate ciphertext and key from circuit + public inputs
pub fn encap_command(
    circuit_path: &str,
    input_path: &str,
    ciphertext_path: &str,
    key_path: &str,
) -> Result<()> {
    println!("📂 Loading R1CS circuit...");
    let r1cs = R1csFile::from_file(circuit_path).context("Failed to load R1CS circuit")?;

    println!("   - Constraints: {}", r1cs.n_constraints);
    println!("   - Public inputs: {}", r1cs.n_pub_in);
    println!("   - Wires: {}", r1cs.n_wires);

    // Parse public inputs from JSON
    println!("\n📋 Loading public inputs from JSON...");
    let input_json = fs::read_to_string(input_path).context("Failed to read input JSON file")?;
    let inputs = parse_circuit_inputs(&input_json).context("Failed to parse input JSON")?;

    println!("   - Parsed {} field elements", inputs.len());

    // Create witness map with constant (wire 0) and public inputs
    let mut witness = HashMap::new();
    witness.insert(0, Fr::from(1u64)); // Wire 0 is always 1 (constant)

    for (i, value) in inputs.iter().enumerate() {
        witness.insert((i + 1) as u32, *value);
    }

    println!("\n🔐 Running Encap...");
    let mut circuit =
        CircomCircuit::from_r1cs(&circuit_path).context("Failed to create circuit")?;
    circuit.set_witness(witness);

    // Setup RNG
    let mut rng = ark_std::rand::rngs::OsRng;

    // Call zkenc-core encap
    let (ciphertext, key) = encap::<Bn254, _, _>(circuit, &mut rng)
        .map_err(|e| anyhow::anyhow!("Encap failed: {:?}", e))?;

    // Serialize and save ciphertext
    println!("\n💾 Saving ciphertext...");
    let mut ciphertext_bytes = Vec::new();
    ciphertext
        .serialize_compressed(&mut ciphertext_bytes)
        .context("Failed to serialize ciphertext")?;

    fs::write(ciphertext_path, &ciphertext_bytes).context("Failed to write ciphertext file")?;
    println!("   ✅ Ciphertext saved ({} bytes)", ciphertext_bytes.len());

    // Serialize and save key
    println!("\n🔑 Saving key...");
    let mut key_bytes = Vec::new();
    key.serialize_compressed(&mut key_bytes)
        .context("Failed to serialize key")?;

    fs::write(key_path, &key_bytes).context("Failed to write key file")?;
    println!("   ✅ Key saved ({} bytes)", key_bytes.len());

    Ok(())
}

/// Decap command: Recover key from circuit + witness + ciphertext
pub fn decap_command(
    circuit_path: &str,
    witness_path: &str,
    ciphertext_path: &str,
    key_path: &str,
) -> Result<()> {
    println!("📂 Loading R1CS circuit...");
    let r1cs = R1csFile::from_file(circuit_path).context("Failed to load R1CS circuit")?;

    println!("   - Constraints: {}", r1cs.n_constraints);
    println!("   - Public inputs: {}", r1cs.n_pub_in);
    println!("   - Wires: {}", r1cs.n_wires);

    // Load witness from snarkjs .wtns file
    println!("\n📋 Loading witness from snarkjs...");
    let witness_file =
        WitnessFile::from_file(witness_path).context("Failed to load witness file")?;

    println!("   - Witness elements: {}", witness_file.n_witness);

    // Convert witness to field elements
    let witness = witness_file
        .to_field_elements::<Fr>()
        .context("Failed to convert witness to field elements")?;

    // Load ciphertext
    println!("\n📦 Loading ciphertext...");
    let ciphertext_bytes = fs::read(ciphertext_path).context("Failed to read ciphertext file")?;

    let ciphertext = Ciphertext::<Bn254>::deserialize_compressed(&ciphertext_bytes[..])
        .context("Failed to deserialize ciphertext")?;

    println!("   - Ciphertext size: {} bytes", ciphertext_bytes.len());

    // Create circuit with full witness
    println!("\n🔓 Running Decap...");
    let mut circuit =
        CircomCircuit::from_r1cs(&circuit_path).context("Failed to create circuit")?;
    circuit.set_witness(witness);

    // Call zkenc-core decap
    let key = decap::<Bn254, _>(circuit, &ciphertext)
        .map_err(|e| anyhow::anyhow!("Decap failed: {:?}", e))?;

    // Serialize and save key
    println!("\n🔑 Saving recovered key...");
    let mut key_bytes = Vec::new();
    key.serialize_compressed(&mut key_bytes)
        .context("Failed to serialize key")?;

    fs::write(key_path, &key_bytes).context("Failed to write key file")?;
    println!("   ✅ Key saved ({} bytes)", key_bytes.len());

    Ok(())
}

/// Encrypt command: High-level encryption with combined ciphertext format
/// This format is compatible with zkenc-js encrypt() function
pub fn encrypt_command(
    circuit_path: &str,
    input_path: &str,
    message_path: &str,
    output_path: &str,
    include_public_input: bool,
) -> Result<()> {
    // Step 1: Run encap to get witness ciphertext and key
    println!("🔐 Step 1: Running Encap...");
    println!("� Loading R1CS circuit...");
    let r1cs = R1csFile::from_file(circuit_path).context("Failed to load R1CS circuit")?;

    println!("   - Constraints: {}", r1cs.n_constraints);
    println!("   - Public inputs: {}", r1cs.n_pub_in);
    println!("   - Wires: {}", r1cs.n_wires);

    // Parse public inputs from JSON
    println!("\n📋 Loading public inputs from JSON...");
    let input_json = fs::read_to_string(input_path).context("Failed to read input JSON file")?;
    let inputs = parse_circuit_inputs(&input_json).context("Failed to parse input JSON")?;

    println!("   - Parsed {} field elements", inputs.len());

    // Create witness map with constant (wire 0) and public inputs
    let mut witness = HashMap::new();
    witness.insert(0, Fr::from(1u64)); // Wire 0 is always 1 (constant)

    for (i, value) in inputs.iter().enumerate() {
        witness.insert((i + 1) as u32, *value);
    }

    let mut circuit =
        CircomCircuit::from_r1cs(&circuit_path).context("Failed to create circuit")?;
    circuit.set_witness(witness);

    // Setup RNG
    let mut rng = ark_std::rand::rngs::OsRng;

    // Call zkenc-core encap
    let (ciphertext, key) = encap::<Bn254, _, _>(circuit, &mut rng)
        .map_err(|e| anyhow::anyhow!("Encap failed: {:?}", e))?;

    // Serialize witness ciphertext
    let mut witness_ct_bytes = Vec::new();
    ciphertext
        .serialize_compressed(&mut witness_ct_bytes)
        .context("Failed to serialize ciphertext")?;

    println!(
        "   ✅ Witness ciphertext generated ({} bytes)",
        witness_ct_bytes.len()
    );

    // Step 2: Encrypt message with key
    println!("\n� Step 2: Encrypting message...");
    let message = fs::read(message_path).context("Failed to read message file")?;
    println!("   - Message size: {} bytes", message.len());

    let encrypted_message =
        crypto::encrypt_gcm(key.as_bytes(), &message).context("Message encryption failed")?;
    println!("   ✅ Message encrypted ({} bytes)", encrypted_message.len());

    // Step 3: Combine into zkenc-js compatible format
    println!("\n� Step 3: Creating combined ciphertext...");

    // Prepare public input bytes if needed
    let public_input_bytes = if include_public_input {
        input_json.as_bytes().to_vec()
    } else {
        Vec::new()
    };

    // Calculate total size
    // Format: [flag(1)][witnessLen(4)][witnessCT][publicLen(4)?][publicInput?][encryptedMsg]
    let flag: u8 = if include_public_input { 1 } else { 0 };
    let header_size = if include_public_input { 9 } else { 5 };
    let total_size = header_size
        + witness_ct_bytes.len()
        + public_input_bytes.len()
        + encrypted_message.len();

    let mut combined = Vec::with_capacity(total_size);

    // Write flag
    combined.push(flag);

    // Write witness ciphertext length (big-endian u32)
    combined.extend_from_slice(&(witness_ct_bytes.len() as u32).to_be_bytes());

    // Write witness ciphertext
    combined.extend_from_slice(&witness_ct_bytes);

    // Write public input if included
    if include_public_input {
        combined.extend_from_slice(&(public_input_bytes.len() as u32).to_be_bytes());
        combined.extend_from_slice(&public_input_bytes);
    }

    // Write encrypted message
    combined.extend_from_slice(&encrypted_message);

    // Save combined ciphertext
    fs::write(output_path, &combined).context("Failed to write combined ciphertext")?;
    println!("   ✅ Combined ciphertext saved ({} bytes)", combined.len());

    if include_public_input {
        println!("\n✨ Encryption complete! Public inputs are embedded in the ciphertext.");
    } else {
        println!("\n✨ Encryption complete! Remember to share the public inputs separately.");
    }

    Ok(())
}

/// Decrypt command: High-level decryption from combined ciphertext format
/// This format is compatible with zkenc-js decrypt() function
pub fn decrypt_command(
    circuit_path: &str,
    witness_path: &str,
    ciphertext_path: &str,
    output_path: &str,
) -> Result<()> {
    // Step 1: Parse combined ciphertext
    println!("� Step 1: Parsing combined ciphertext...");
    let combined = fs::read(ciphertext_path).context("Failed to read ciphertext file")?;

    if combined.len() < 5 {
        anyhow::bail!("Invalid ciphertext: too short");
    }

    let mut offset = 0;

    // Read flag
    let flag = combined[offset];
    offset += 1;

    // Read witness ciphertext length
    let witness_len = u32::from_be_bytes([
        combined[offset],
        combined[offset + 1],
        combined[offset + 2],
        combined[offset + 3],
    ]) as usize;
    offset += 4;

    if combined.len() < offset + witness_len {
        anyhow::bail!("Invalid ciphertext: witness length mismatch");
    }

    // Extract witness ciphertext
    let witness_ct_bytes = &combined[offset..offset + witness_len];
    offset += witness_len;

    println!("   - Flag: {}", flag);
    println!("   - Witness ciphertext: {} bytes", witness_len);

    // Skip public input if present (flag === 1)
    if flag == 1 {
        if combined.len() < offset + 4 {
            anyhow::bail!("Invalid ciphertext: missing public input length");
        }

        let public_len = u32::from_be_bytes([
            combined[offset],
            combined[offset + 1],
            combined[offset + 2],
            combined[offset + 3],
        ]) as usize;
        offset += 4;

        if combined.len() < offset + public_len {
            anyhow::bail!("Invalid ciphertext: public input length mismatch");
        }

        // Extract and display public input
        let public_input = &combined[offset..offset + public_len];
        let public_str = String::from_utf8_lossy(public_input);
        println!("   - Public input: {}", public_str);

        offset += public_len;
    }

    // Extract encrypted message
    let encrypted_message = &combined[offset..];
    println!(
        "   - Encrypted message: {} bytes",
        encrypted_message.len()
    );

    // Step 2: Load circuit and witness
    println!("\n🔓 Step 2: Running Decap...");
    println!("📂 Loading R1CS circuit...");
    let r1cs = R1csFile::from_file(circuit_path).context("Failed to load R1CS circuit")?;

    println!("   - Constraints: {}", r1cs.n_constraints);
    println!("   - Public inputs: {}", r1cs.n_pub_in);

    // Load witness from snarkjs .wtns file
    println!("\n📋 Loading witness from snarkjs...");
    let witness_file =
        WitnessFile::from_file(witness_path).context("Failed to load witness file")?;

    println!("   - Witness elements: {}", witness_file.n_witness);

    // Convert witness to field elements
    let witness = witness_file
        .to_field_elements::<Fr>()
        .context("Failed to convert witness to field elements")?;

    // Deserialize witness ciphertext
    let witness_ciphertext = Ciphertext::<Bn254>::deserialize_compressed(&witness_ct_bytes[..])
        .context("Failed to deserialize witness ciphertext")?;

    // Create circuit with full witness
    let mut circuit =
        CircomCircuit::from_r1cs(&circuit_path).context("Failed to create circuit")?;
    circuit.set_witness(witness);

    // Call zkenc-core decap to recover key
    let key = decap::<Bn254, _>(circuit, &witness_ciphertext)
        .map_err(|e| anyhow::anyhow!("Decap failed: {:?}", e))?;

    println!("   ✅ Key recovered from witness");

    // Step 3: Decrypt message with recovered key
    println!("\n🔓 Step 3: Decrypting message...");
    let plaintext = crypto::decrypt_gcm(key.as_bytes(), encrypted_message)
        .context("Message decryption failed")?;

    // Save decrypted message
    fs::write(output_path, &plaintext).context("Failed to write decrypted file")?;
    println!("   ✅ Decrypted message saved ({} bytes)", plaintext.len());

    println!("\n✨ Decryption complete!");

    Ok(())
}

/// Parse circuit inputs from JSON file
/// Returns a vector of field elements in order (flattened if nested)
fn parse_circuit_inputs(json_str: &str) -> Result<Vec<Fr>> {
    let value: serde_json::Value = serde_json::from_str(json_str).context("Invalid JSON")?;

    let obj = value.as_object().context("JSON must be an object")?;

    let mut result = Vec::new();

    // Flatten all values in the JSON object
    fn flatten_value(val: &serde_json::Value, result: &mut Vec<Fr>) -> Result<()> {
        match val {
            serde_json::Value::Number(n) => {
                let num = if let Some(u) = n.as_u64() {
                    Fr::from(u)
                } else if let Some(i) = n.as_i64() {
                    Fr::from(i as u64)
                } else {
                    anyhow::bail!("Unsupported number format");
                };
                result.push(num);
            }
            serde_json::Value::String(s) => {
                // Try to parse as number
                let num = s
                    .parse::<u64>()
                    .context("Failed to parse string as number")?;
                result.push(Fr::from(num));
            }
            serde_json::Value::Array(arr) => {
                for item in arr {
                    flatten_value(item, result)?;
                }
            }
            _ => anyhow::bail!("Unsupported JSON type"),
        }
        Ok(())
    }

    // Process in sorted key order for consistency
    let mut keys: Vec<_> = obj.keys().collect();
    keys.sort();

    for key in keys {
        if let Some(val) = obj.get(key) {
            flatten_value(val, &mut result)?;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_inputs() {
        let json = r#"{
            "a": 5,
            "b": [1, 2, 3],
            "c": "42"
        }"#;

        let inputs = parse_circuit_inputs(json).unwrap();
        assert_eq!(inputs.len(), 5); // a(1) + b(3) + c(1)
        assert_eq!(inputs[0], Fr::from(5u64)); // a
        assert_eq!(inputs[1], Fr::from(1u64)); // b[0]
        assert_eq!(inputs[2], Fr::from(2u64)); // b[1]
        assert_eq!(inputs[3], Fr::from(3u64)); // b[2]
        assert_eq!(inputs[4], Fr::from(42u64)); // c
    }
}
