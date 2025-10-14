// witness.rs - Parse Circom witness files (.wtns format)
//
// The .wtns format is a binary format produced by snarkjs for storing witness values.
// Format spec (from snarkjs/circom):
//   - Magic: "wtns" (4 bytes)
//   - Version: 2 (u32 LE)
//   - Sections:
//     - Section 1: Header (field size, prime, witness count)
//     - Section 2: Witness data (array of field elements)

use anyhow::{bail, Context, Result};
use ark_ff::PrimeField;
use ark_serialize::CanonicalDeserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};

const WTNS_MAGIC: &[u8; 4] = b"wtns";

/// Parsed witness file containing wire assignments
#[derive(Debug, Clone)]
pub struct WitnessFile {
    /// Field prime modulus (for validation)
    pub prime: Vec<u8>,
    /// Number of witness elements
    pub n_witness: u32,
    /// Wire assignments: wire_id -> field element bytes (little-endian)
    pub assignments: HashMap<u32, Vec<u8>>,
}

impl WitnessFile {
    /// Load witness from .wtns file
    pub fn from_file(path: &str) -> Result<Self> {
        let file =
            File::open(path).with_context(|| format!("Failed to open witness file: {}", path))?;
        let mut reader = BufReader::new(file);

        // Read magic
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;
        if &magic != WTNS_MAGIC {
            bail!("Invalid wtns magic bytes: expected 'wtns', got {:?}", magic);
        }

        // Read version
        let version = read_u32_le(&mut reader)?;
        if version != 2 {
            bail!("Unsupported wtns version: {}", version);
        }

        // Read number of sections
        let n_sections = read_u32_le(&mut reader)?;

        let mut prime = Vec::new();
        let mut n_witness = 0u32;
        let mut witness_data = Vec::new();

        // Read all sections
        for _ in 0..n_sections {
            let section_type = read_u32_le(&mut reader)?;
            let section_size = read_u64_le(&mut reader)?;

            match section_type {
                1 => {
                    // Header section
                    let field_size = read_u32_le(&mut reader)?;

                    // Read prime modulus
                    prime = vec![0u8; field_size as usize];
                    reader.read_exact(&mut prime)?;

                    // Read witness count
                    n_witness = read_u32_le(&mut reader)?;
                }
                2 => {
                    // Witness data section
                    witness_data = vec![0u8; section_size as usize];
                    reader.read_exact(&mut witness_data)?;
                }
                _ => {
                    // Skip unknown sections
                    let mut skip = vec![0u8; section_size as usize];
                    reader.read_exact(&mut skip)?;
                }
            }
        }

        if prime.is_empty() {
            bail!("Missing header section in wtns file");
        }
        if witness_data.is_empty() {
            bail!("Missing witness data section in wtns file");
        }

        // Parse witness data into assignments
        let field_size = prime.len();
        let expected_size = n_witness as usize * field_size;
        if witness_data.len() != expected_size {
            bail!(
                "Witness data size mismatch: expected {} bytes ({} elements * {} bytes), got {}",
                expected_size,
                n_witness,
                field_size,
                witness_data.len()
            );
        }

        let mut assignments = HashMap::new();
        for i in 0..n_witness {
            let start = (i as usize) * field_size;
            let end = start + field_size;
            let value = witness_data[start..end].to_vec();
            assignments.insert(i, value);
        }

        Ok(WitnessFile {
            prime,
            n_witness,
            assignments,
        })
    }

    /// Convert to field elements for a specific prime field
    pub fn to_field_elements<F: PrimeField>(&self) -> Result<HashMap<u32, F>> {
        let mut result = HashMap::new();

        for (&wire_id, bytes) in &self.assignments {
            // Deserialize from little-endian bytes
            let value = F::deserialize_uncompressed(&bytes[..]).with_context(|| {
                format!("Failed to deserialize wire {} as field element", wire_id)
            })?;
            result.insert(wire_id, value);
        }

        Ok(result)
    }

    /// Get the number of wires in the witness
    pub fn n_wires(&self) -> u32 {
        self.n_witness
    }
}

// Helper functions for reading little-endian integers

fn read_u32_le<R: Read>(reader: &mut R) -> Result<u32> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

fn read_u64_le<R: Read>(reader: &mut R) -> Result<u64> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    Ok(u64::from_le_bytes(buf))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bls12_381::Fr;

    #[test]
    #[ignore] // Only run when witness files are available
    fn test_load_sudoku_witness() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/inputs/sudoku_sudoku_basic.wtns"
        );

        let witness = WitnessFile::from_file(path).expect("Failed to load witness file");

        println!("Loaded witness:");
        println!("  - n_witness: {}", witness.n_witness);
        println!("  - prime size: {} bytes", witness.prime.len());
        println!("  - wire 0 (constant): {:?}", witness.assignments.get(&0));
        println!(
            "  - wire 1 (first input): {:?}",
            witness.assignments.get(&1)
        );

        // Convert to field elements
        let field_elements: HashMap<u32, Fr> = witness
            .to_field_elements()
            .expect("Failed to convert to field elements");

        assert_eq!(field_elements.len(), witness.n_witness as usize);

        // Wire 0 should be 1 (constant)
        assert_eq!(field_elements[&0], Fr::from(1u64));

        println!(
            "✅ Successfully converted {} wires to field elements",
            field_elements.len()
        );
    }

    #[test]
    #[ignore]
    fn test_load_signature_witness() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/inputs/signature_signature_basic.wtns"
        );

        let witness = WitnessFile::from_file(path).expect("Failed to load witness file");

        println!("Loaded signature witness:");
        println!("  - n_witness: {}", witness.n_witness);
        println!("  - prime size: {} bytes", witness.prime.len());

        let field_elements: HashMap<u32, Fr> = witness
            .to_field_elements()
            .expect("Failed to convert to field elements");

        assert_eq!(field_elements[&0], Fr::from(1u64));

        println!(
            "✅ Successfully loaded signature witness with {} wires",
            field_elements.len()
        );
    }
}
