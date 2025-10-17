//! zkenc-js: WASM bindings for zkenc-core
//!
//! This module provides JavaScript/WASM interface for witness encryption.
//! It implements R1CS parsing and circuit construction to work with Circom circuits.
//!
//! Note: This duplicates parsing logic from zkenc-cli for independence -
//! zkenc-js and zkenc-cli are parallel consumers of zkenc-core.

use ark_bn254::{Bn254, Fr};
use ark_ff::PrimeField;
use ark_relations::gr1cs::{
    ConstraintSynthesizer, ConstraintSystemRef, LinearCombination, SynthesisError, Variable,
    R1CS_PREDICATE_LABEL,
};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::rand::rngs::StdRng;
use ark_std::rand::SeedableRng;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use zkenc_core::{decap, encap, Ciphertext};

/// Initialize WASM module with better error messages
#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Result of encapsulation containing ciphertext and key
#[wasm_bindgen]
pub struct EncapResult {
    ciphertext: Vec<u8>,
    key: Vec<u8>,
}

#[wasm_bindgen]
impl EncapResult {
    #[wasm_bindgen(getter)]
    pub fn ciphertext(&self) -> Vec<u8> {
        self.ciphertext.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn key(&self) -> Vec<u8> {
        self.key.clone()
    }
}

//////////////////////////////////////////////////////////////////////////////
// R1CS Parsing (duplicated from zkenc-cli for independence)
//////////////////////////////////////////////////////////////////////////////

struct R1csHeader {
    field_size: u32,
    n_wires: u32,
    n_pub_out: u32,
    n_pub_in: u32,
    n_constraints: u32,
}

impl R1csHeader {
    fn n_public_inputs(&self) -> u32 {
        self.n_pub_out + self.n_pub_in
    }
}

struct R1csConstraint {
    a_factors: Vec<(u32, Vec<u8>)>,
    b_factors: Vec<(u32, Vec<u8>)>,
    c_factors: Vec<(u32, Vec<u8>)>,
}

fn parse_r1cs(data: &[u8]) -> Result<(R1csHeader, Vec<R1csConstraint>), String> {
    let mut pos = 0;

    // Helper to read u32
    let read_u32 = |pos: &mut usize| -> Result<u32, String> {
        if *pos + 4 > data.len() {
            return Err("Unexpected end of data".to_string());
        }
        let val = u32::from_le_bytes([data[*pos], data[*pos + 1], data[*pos + 2], data[*pos + 3]]);
        *pos += 4;
        Ok(val)
    };

    // Helper to read u64
    let read_u64 = |pos: &mut usize| -> Result<u64, String> {
        if *pos + 8 > data.len() {
            return Err("Unexpected end of data".to_string());
        }
        let val = u64::from_le_bytes([
            data[*pos],
            data[*pos + 1],
            data[*pos + 2],
            data[*pos + 3],
            data[*pos + 4],
            data[*pos + 5],
            data[*pos + 6],
            data[*pos + 7],
        ]);
        *pos += 8;
        Ok(val)
    };

    // Check magic "r1cs"
    if pos + 4 > data.len() || &data[pos..pos + 4] != b"r1cs" {
        return Err("Invalid R1CS file: wrong magic".to_string());
    }
    pos += 4;

    // Version must be 1
    let version = read_u32(&mut pos)?;
    if version != 1 {
        return Err(format!("Unsupported R1CS version: {}", version));
    }

    // Number of sections
    let n_sections = read_u32(&mut pos)?;

    // First pass: collect all section positions
    let mut sections = Vec::new();
    for _ in 0..n_sections {
        let section_type = read_u32(&mut pos)?;
        let section_len = read_u64(&mut pos)? as usize;
        let section_start = pos;
        sections.push((section_type, section_len, section_start));
        pos = section_start + section_len;
    }

    // Second pass: find and parse header section first
    let header = {
        let header_section = sections
            .iter()
            .find(|(t, _, _)| *t == 0x01)
            .ok_or("Header section (type 1) not found")?;

        let mut header_pos = header_section.2;
        let field_size = read_u32(&mut header_pos)?;
        let prime_len = field_size as usize;
        if header_pos + prime_len > data.len() {
            return Err("Invalid prime length".to_string());
        }
        header_pos += prime_len; // Skip prime bytes

        let n_wires = read_u32(&mut header_pos)?;
        let n_pub_out = read_u32(&mut header_pos)?;
        let n_pub_in = read_u32(&mut header_pos)?;
        let _n_prv_in = read_u32(&mut header_pos)?;
        let _n_labels = read_u64(&mut header_pos)?;
        let n_constraints = read_u32(&mut header_pos)?;

        R1csHeader {
            field_size,
            n_wires,
            n_pub_out,
            n_pub_in,
            n_constraints,
        }
    };

    // Third pass: parse constraints section
    let constraints = {
        let constraints_section = sections
            .iter()
            .find(|(t, _, _)| *t == 0x02)
            .ok_or("Constraints section (type 2) not found")?;

        let mut constraints_pos = constraints_section.2;
        let mut constraints = Vec::new();

        for _ in 0..header.n_constraints {
            // Parse A linear combination
            let a_factors =
                parse_linear_combination(data, &mut constraints_pos, header.field_size)?;
            // Parse B linear combination
            let b_factors =
                parse_linear_combination(data, &mut constraints_pos, header.field_size)?;
            // Parse C linear combination
            let c_factors =
                parse_linear_combination(data, &mut constraints_pos, header.field_size)?;

            constraints.push(R1csConstraint {
                a_factors,
                b_factors,
                c_factors,
            });
        }

        constraints
    };

    Ok((header, constraints))
}

fn parse_linear_combination(
    data: &[u8],
    pos: &mut usize,
    field_size: u32,
) -> Result<Vec<(u32, Vec<u8>)>, String> {
    if *pos + 4 > data.len() {
        return Err("Unexpected end of data in LC".to_string());
    }
    let n_factors =
        u32::from_le_bytes([data[*pos], data[*pos + 1], data[*pos + 2], data[*pos + 3]]);
    *pos += 4;

    let mut factors = Vec::new();
    for _ in 0..n_factors {
        if *pos + 4 > data.len() {
            return Err("Unexpected end of data reading wire id".to_string());
        }
        let wire_id =
            u32::from_le_bytes([data[*pos], data[*pos + 1], data[*pos + 2], data[*pos + 3]]);
        *pos += 4;

        let value_len = field_size as usize;
        if *pos + value_len > data.len() {
            return Err("Unexpected end of data reading factor value".to_string());
        }
        let value = data[*pos..*pos + value_len].to_vec();
        *pos += value_len;

        factors.push((wire_id, value));
    }

    Ok(factors)
}

//////////////////////////////////////////////////////////////////////////////
// Witness Parsing (snarkjs wtns format)
//////////////////////////////////////////////////////////////////////////////

fn parse_witness(data: &[u8]) -> Result<Vec<Fr>, String> {
    let mut pos = 0;

    // Check magic "wtns"
    if pos + 4 > data.len() || &data[pos..pos + 4] != b"wtns" {
        return Err("Invalid witness file: wrong magic".to_string());
    }
    pos += 4;

    // Version
    let version = u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
    pos += 4;
    if version != 2 {
        return Err(format!("Unsupported witness version: {}", version));
    }

    // Number of sections
    let n_sections = u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
    pos += 4;

    let mut witness: Vec<Fr> = Vec::new();
    let mut n8 = 0usize;

    for _ in 0..n_sections {
        let section_type =
            u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
        pos += 4;

        let section_len = u64::from_le_bytes([
            data[pos],
            data[pos + 1],
            data[pos + 2],
            data[pos + 3],
            data[pos + 4],
            data[pos + 5],
            data[pos + 6],
            data[pos + 7],
        ]) as usize;
        pos += 8;

        let section_end = pos + section_len;

        if section_type == 1 {
            // Header section
            n8 = u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]])
                as usize;
        } else if section_type == 2 {
            // Witness values section - contains raw witness data (field_size * n_witness bytes)
            // Read all witness values directly
            while pos + n8 <= section_end {
                let mut bytes = vec![0u8; 32];
                let copy_len = n8.min(32);
                bytes[..copy_len].copy_from_slice(&data[pos..pos + copy_len]);

                witness.push(Fr::from_le_bytes_mod_order(&bytes));
                pos += n8;
            }
        }

        pos = section_end;
    }

    Ok(witness)
}

//////////////////////////////////////////////////////////////////////////////
// CircomCircuit implementation
//////////////////////////////////////////////////////////////////////////////

struct CircomCircuit {
    header: R1csHeader,
    constraints: Vec<R1csConstraint>,
    witness: HashMap<u32, Fr>,
}

impl CircomCircuit {
    fn bytes_to_fr(bytes: &[u8]) -> Fr {
        let mut bytes_array = [0u8; 32];
        let len = bytes.len().min(32);
        bytes_array[..len].copy_from_slice(&bytes[..len]);
        Fr::from_le_bytes_mod_order(&bytes_array)
    }
}

impl ConstraintSynthesizer<Fr> for CircomCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        // Allocate all variables
        let mut variables: HashMap<u32, Variable> = HashMap::new();
        variables.insert(0, Variable::One);

        // Allocate public inputs
        let n_public = self.header.n_public_inputs();
        for wire_id in 1..=n_public {
            let value = self.witness.get(&wire_id).copied();
            let var = cs.new_input_variable(|| value.ok_or(SynthesisError::AssignmentMissing))?;
            variables.insert(wire_id, var);
        }

        // Allocate private witnesses
        for wire_id in (n_public + 1)..self.header.n_wires {
            let value = self.witness.get(&wire_id).copied();
            let var = cs.new_witness_variable(|| value.ok_or(SynthesisError::AssignmentMissing))?;
            variables.insert(wire_id, var);
        }

        // Add constraints
        for constraint in self.constraints {
            let a_lc = build_lc(&constraint.a_factors, &variables);
            let b_lc = build_lc(&constraint.b_factors, &variables);
            let c_lc = build_lc(&constraint.c_factors, &variables);

            let boxed: Vec<Box<dyn FnOnce() -> LinearCombination<Fr>>> = vec![
                Box::new(move || a_lc),
                Box::new(move || b_lc),
                Box::new(move || c_lc),
            ];
            cs.enforce_constraint(R1CS_PREDICATE_LABEL, boxed)?;
        }

        Ok(())
    }
}

fn build_lc(
    factors: &[(u32, Vec<u8>)],
    variables: &HashMap<u32, Variable>,
) -> LinearCombination<Fr> {
    let mut lc = LinearCombination::zero();
    for (wire_id, coeff_bytes) in factors {
        if let Some(&var) = variables.get(wire_id) {
            let coeff = CircomCircuit::bytes_to_fr(coeff_bytes);
            lc = lc + (coeff, var);
        }
    }
    lc
}

//////////////////////////////////////////////////////////////////////////////
// WASM API
//////////////////////////////////////////////////////////////////////////////

/// Perform encapsulation with R1CS circuit and public inputs
///
/// # Arguments
/// * `r1cs_bytes` - R1CS circuit file bytes
/// * `public_inputs_json` - JSON string with public inputs (no witness)
///
/// # Returns
/// Ciphertext and 32-byte symmetric key
#[wasm_bindgen]
pub fn wasm_encap(r1cs_bytes: &[u8], public_inputs_json: &str) -> Result<EncapResult, JsValue> {
    // Parse R1CS
    let (header, constraints) = parse_r1cs(r1cs_bytes)
        .map_err(|e| JsValue::from_str(&format!("R1CS parse error: {}", e)))?;

    // Parse public inputs from JSON
    let public_inputs: serde_json::Value = serde_json::from_str(public_inputs_json)
        .map_err(|e| JsValue::from_str(&format!("JSON parse error: {}", e)))?;

    // Flatten JSON to field elements
    let mut public_values = Vec::new();
    flatten_json(&public_inputs, &mut public_values).map_err(|e| JsValue::from_str(&e))?;

    // Ensure we have exactly n_public_inputs values
    let n_pub = header.n_public_inputs() as usize;
    if public_values.len() < n_pub {
        return Err(JsValue::from_str(&format!(
            "Not enough public inputs: expected {}, got {}",
            n_pub,
            public_values.len()
        )));
    }
    public_values.truncate(n_pub);

    // Create witness map with only public inputs (for encap)
    // Wire 0 = 1 (constant), Wire 1..n_pub = public inputs
    // Private wires are intentionally left unassigned
    let mut witness_map = HashMap::new();
    witness_map.insert(0, Fr::from(1u64)); // wire 0 = 1
    for (i, value) in public_values.iter().enumerate() {
        witness_map.insert((i + 1) as u32, *value);
    }

    // Create circuit with only public inputs assigned
    let circuit = CircomCircuit {
        header,
        constraints,
        witness: witness_map,
    };

    // Generate random seed
    let mut seed = [0u8; 32];
    getrandom::getrandom(&mut seed)
        .map_err(|e| JsValue::from_str(&format!("Random generation failed: {}", e)))?;
    let mut rng = StdRng::from_seed(seed);

    // Perform encapsulation
    let (ciphertext, key) = encap::<Bn254, _, _>(circuit, &mut rng)
        .map_err(|e| JsValue::from_str(&format!("Encapsulation failed: {:?}", e)))?;

    // Serialize ciphertext
    let mut ct_bytes = Vec::new();
    ciphertext
        .serialize_compressed(&mut ct_bytes)
        .map_err(|e| JsValue::from_str(&format!("Serialization failed: {:?}", e)))?;

    Ok(EncapResult {
        ciphertext: ct_bytes,
        key: key.0.to_vec(),
    })
}

/// Perform decapsulation with R1CS circuit, witness, and ciphertext
///
/// # Arguments
/// * `r1cs_bytes` - R1CS circuit file bytes
/// * `witness_bytes` - Witness file bytes (snarkjs wtns format)
/// * `ciphertext_bytes` - Ciphertext from encapsulation
///
/// # Returns
/// 32-byte symmetric key
#[wasm_bindgen]
pub fn wasm_decap(
    r1cs_bytes: &[u8],
    witness_bytes: &[u8],
    ciphertext_bytes: &[u8],
) -> Result<Vec<u8>, JsValue> {
    // Parse R1CS
    let (header, constraints) = parse_r1cs(r1cs_bytes)
        .map_err(|e| JsValue::from_str(&format!("R1CS parse error: {}", e)))?;

    // Parse witness
    let witness_values = parse_witness(witness_bytes)
        .map_err(|e| JsValue::from_str(&format!("Witness parse error: {}", e)))?;

    if witness_values.len() != header.n_wires as usize {
        return Err(JsValue::from_str(&format!(
            "Witness size mismatch: expected {}, got {}",
            header.n_wires,
            witness_values.len()
        )));
    }

    // Create witness map with all values (for decap)
    let mut witness_map = HashMap::new();
    for (idx, val) in witness_values.iter().enumerate() {
        witness_map.insert(idx as u32, *val);
    }

    // Create circuit with full witness
    let circuit = CircomCircuit {
        header,
        constraints,
        witness: witness_map,
    };

    // Deserialize ciphertext
    let ciphertext = Ciphertext::<Bn254>::deserialize_compressed(ciphertext_bytes)
        .map_err(|e| JsValue::from_str(&format!("Ciphertext deserialization failed: {:?}", e)))?;

    // Perform decapsulation
    let key = decap::<Bn254, _>(circuit, &ciphertext)
        .map_err(|e| JsValue::from_str(&format!("Decapsulation failed: {:?}", e)))?;

    Ok(key.0.to_vec())
}

/// Helper: Flatten JSON to field elements
fn flatten_json(value: &serde_json::Value, result: &mut Vec<Fr>) -> Result<(), String> {
    match value {
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_u64() {
                result.push(Fr::from(i));
            } else if let Some(i) = n.as_i64() {
                if i < 0 {
                    return Err(format!("Negative numbers not supported: {}", i));
                }
                result.push(Fr::from(i as u64));
            } else {
                return Err(format!("Invalid number: {}", n));
            }
            Ok(())
        }
        serde_json::Value::String(s) => {
            let num = s
                .parse::<u64>()
                .map_err(|_| format!("Failed to parse string as number: {}", s))?;
            result.push(Fr::from(num));
            Ok(())
        }
        serde_json::Value::Array(arr) => {
            for item in arr {
                flatten_json(item, result)?;
            }
            Ok(())
        }
        serde_json::Value::Object(obj) => {
            for (_key, val) in obj {
                flatten_json(val, result)?;
            }
            Ok(())
        }
        _ => Err("Unsupported JSON value type".to_string()),
    }
}
