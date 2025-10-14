// commands.rs - CLI command implementations
//
// This module implements the actual logic for CLI commands:
// - encap: Generate ciphertext and key from circuit + public inputs
// - decap: Recover key from circuit + witness + ciphertext
// - encrypt: Encrypt message with key using AES-GCM
// - decrypt: Decrypt message with key using AES-GCM

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

/// Encrypt command: Encrypt message with key using AES-256-GCM
pub fn encrypt_command(key_path: &str, input_path: &str, output_path: &str) -> Result<()> {
    // Load key
    println!("🔑 Loading key...");
    let key_bytes = fs::read(key_path).context("Failed to read key file")?;

    let key = Key::deserialize_compressed(&key_bytes[..]).context("Failed to deserialize key")?;

    // Load plaintext
    println!("📄 Loading plaintext...");
    let plaintext = fs::read(input_path).context("Failed to read input file")?;
    println!("   - Plaintext size: {} bytes", plaintext.len());

    // Encrypt using AES-256-GCM
    println!("\n🔒 Encrypting...");
    let ciphertext =
        crypto::encrypt_gcm(key.as_bytes(), &plaintext).context("Encryption failed")?;

    // Save encrypted data
    fs::write(output_path, &ciphertext).context("Failed to write encrypted file")?;
    println!("   ✅ Encrypted file saved ({} bytes)", ciphertext.len());

    Ok(())
}

/// Decrypt command: Decrypt message with key using AES-256-GCM
pub fn decrypt_command(key_path: &str, input_path: &str, output_path: &str) -> Result<()> {
    // Load key
    println!("🔑 Loading key...");
    let key_bytes = fs::read(key_path).context("Failed to read key file")?;

    let key = Key::deserialize_compressed(&key_bytes[..]).context("Failed to deserialize key")?;

    // Load ciphertext
    println!("📦 Loading encrypted data...");
    let ciphertext = fs::read(input_path).context("Failed to read encrypted file")?;
    println!("   - Encrypted size: {} bytes", ciphertext.len());

    // Decrypt using AES-256-GCM
    println!("\n🔓 Decrypting...");
    let plaintext =
        crypto::decrypt_gcm(key.as_bytes(), &ciphertext).context("Decryption failed")?;

    // Save decrypted data
    fs::write(output_path, &plaintext).context("Failed to write decrypted file")?;
    println!("   ✅ Decrypted file saved ({} bytes)", plaintext.len());

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
