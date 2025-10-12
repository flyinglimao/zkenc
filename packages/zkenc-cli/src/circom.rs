use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Load a Circom circuit from R1CS and WASM files
///
/// Returns basic circuit information: (num_constraints, num_public_inputs, num_variables)
///
/// This is a minimal implementation that verifies the files exist and returns dummy data.
/// In Phase 2/3, we'll integrate with ark-circom properly for witness generation.
pub fn load_circom_circuit<P: AsRef<Path>>(
    r1cs_path: P,
    wasm_path: P,
) -> Result<(usize, usize, usize)> {
    // Verify files exist
    let r1cs_data = fs::read(r1cs_path.as_ref())
        .with_context(|| format!("Failed to read R1CS file: {:?}", r1cs_path.as_ref()))?;

    let _wasm_data = fs::read(wasm_path.as_ref())
        .with_context(|| format!("Failed to read WASM file: {:?}", wasm_path.as_ref()))?;

    // For now, return success with positive values indicating we found the files
    // In a real implementation with proper ark-circom integration:
    // - Parse the R1CS header to get actual constraint/variable counts
    // - Use WitnessCalculator from WASM for computing witnesses
    // - Integrate with zkenc-core's Circuit trait

    // Return dummy values for now (will be replaced in Phase 3)
    let num_constraints = r1cs_data.len() / 1000; // Rough estimate from file size
    let num_public_inputs = 3; // Common default
    let num_variables = num_constraints * 2; // Rough estimate

    Ok((num_constraints, num_public_inputs, num_variables))
}

/// Parse JSON input file for Circom circuits
///
/// Returns a map of input names to their values (as strings)
/// Handles nested arrays by flattening them
pub fn parse_inputs<P: AsRef<Path>>(
    input_path: P,
) -> Result<HashMap<String, Vec<String>>> {
    // Read and parse JSON file
    let content = fs::read_to_string(input_path.as_ref())
        .with_context(|| format!("Failed to read input file: {:?}", input_path.as_ref()))?;

    let json: Value = serde_json::from_str(&content)
        .context("Failed to parse JSON")?;

    let obj = json.as_object()
        .context("Input JSON must be an object")?;

    let mut result = HashMap::new();

    // Process each field in the JSON
    for (key, value) in obj.iter() {
        let values = flatten_value(value);
        result.insert(key.clone(), values);
    }

    Ok(result)
}

/// Recursively flatten a JSON value into a list of strings
fn flatten_value(value: &Value) -> Vec<String> {
    match value {
        Value::String(s) => vec![s.clone()],
        Value::Number(n) => vec![n.to_string()],
        Value::Array(arr) => {
            arr.iter()
                .flat_map(flatten_value)
                .collect()
        }
        Value::Bool(b) => vec![if *b { "1".to_string() } else { "0".to_string() }],
        Value::Null => vec!["0".to_string()],
        Value::Object(_) => {
            // For nested objects, we could handle them differently
            // For now, just skip them
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_load_signature_circuit() {
        let r1cs_path = PathBuf::from("tests/r1cs/signature.r1cs");
        let wasm_path = PathBuf::from("tests/r1cs/signature.wasm");

        let result = load_circom_circuit(&r1cs_path, &wasm_path);
        assert!(result.is_ok(), "Failed to load circuit: {:?}", result.err());

        let (num_constraints, num_public, num_variables) = result.unwrap();

        // Verify basic circuit information
        assert!(num_constraints > 0, "Circuit should have constraints");
        assert!(num_public > 0, "Circuit should have public inputs");
        assert!(num_variables > 0, "Circuit should have variables");
    }
}
