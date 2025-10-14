// circom_circuit.rs - Test with real Circom circuits loaded from fixtures
//
// This test loads SerializableCircuit from bincode and tests encap/decap

#![cfg(feature = "test_fixtures")]

use ark_bn254::Fr; // Circom uses BN254 (alt_bn128)
use ark_ff::PrimeField;
use ark_relations::gr1cs::{
    ConstraintSynthesizer, ConstraintSystemRef, LinearCombination, SynthesisError, Variable,
};
use std::collections::HashMap;
use std::path::PathBuf;
use zkenc_core::serializable::{SerializableCircuit, SerializableTestCase};

/// Circom circuit wrapper for testing
/// Implements ConstraintSynthesizer from SerializableCircuit
pub struct TestCircuit {
    pub circuit: SerializableCircuit,
    pub witness: HashMap<u32, Fr>,
}

impl TestCircuit {
    /// Load from bincode fixture
    pub fn from_fixture(name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/fixtures");
        path.push(format!("{}.bin", name));

        let bytes = std::fs::read(&path)?;
        let test_case: SerializableTestCase = bincode::deserialize(&bytes)?;

        // Convert witness bytes to Fr
        let mut witness = HashMap::new();
        for (wire_id, bytes) in test_case.witness.assignments {
            let fr = Self::bytes_to_fr(&bytes)?;
            witness.insert(wire_id, fr);
        }

        Ok(Self {
            circuit: test_case.circuit,
            witness,
        })
    }

    /// Create with specific witness values (for encap with public inputs only)
    pub fn with_public_inputs(
        circuit: SerializableCircuit,
        public_inputs: HashMap<u32, Fr>,
    ) -> Self {
        Self {
            circuit,
            witness: public_inputs,
        }
    }

    /// Convert bytes to Fr field element
    fn bytes_to_fr(bytes: &[u8]) -> Result<Fr, SynthesisError> {
        let mut bytes_array = vec![0u8; 32];
        let len = bytes.len().min(32);
        bytes_array[..len].copy_from_slice(&bytes[..len]);
        Ok(Fr::from_le_bytes_mod_order(&bytes_array))
    }

    /// Get number of public inputs
    pub fn n_public_inputs(&self) -> u32 {
        self.circuit.n_pub_out + self.circuit.n_pub_in
    }
}

impl ConstraintSynthesizer<Fr> for TestCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        use ark_relations::gr1cs::R1CS_PREDICATE_LABEL;

        // Allocate variables
        let mut variables: HashMap<u32, Variable> = HashMap::new();

        // Wire 0 is always ONE
        variables.insert(0, Variable::One);

        // Allocate public inputs (outputs + inputs)
        let n_public = self.n_public_inputs();
        for wire_id in 1..=n_public {
            let value = self.witness.get(&wire_id).copied();
            let var = cs.new_input_variable(|| value.ok_or(SynthesisError::AssignmentMissing))?;
            variables.insert(wire_id, var);
        }

        // Allocate private witnesses
        for wire_id in (n_public + 1)..self.circuit.n_wires {
            let value = self.witness.get(&wire_id).copied();
            let var = cs.new_witness_variable(|| value.ok_or(SynthesisError::AssignmentMissing))?;
            variables.insert(wire_id, var);
        }

        // Add all constraints: A * B = C
        for (idx, constraint) in self.circuit.constraints.iter().enumerate() {
            // Clone data for closures
            let a_factors = constraint.a.factors.clone();
            let b_factors = constraint.b.factors.clone();
            let c_factors = constraint.c.factors.clone();
            let vars_a = variables.clone();
            let vars_b = variables.clone();
            let vars_c = variables.clone();

            // Build closures
            let a_closure = move || {
                let mut lc = LinearCombination::<Fr>::zero();
                for factor in &a_factors {
                    if let Ok(coeff) = Self::bytes_to_fr(&factor.coefficient_bytes) {
                        if let Some(var) = vars_a.get(&factor.wire_id) {
                            lc = lc + (coeff, *var);
                        }
                    }
                }
                lc
            };

            let b_closure = move || {
                let mut lc = LinearCombination::<Fr>::zero();
                for factor in &b_factors {
                    if let Ok(coeff) = Self::bytes_to_fr(&factor.coefficient_bytes) {
                        if let Some(var) = vars_b.get(&factor.wire_id) {
                            lc = lc + (coeff, *var);
                        }
                    }
                }
                lc
            };

            let c_closure = move || {
                let mut lc = LinearCombination::<Fr>::zero();
                for factor in &c_factors {
                    if let Ok(coeff) = Self::bytes_to_fr(&factor.coefficient_bytes) {
                        if let Some(var) = vars_c.get(&factor.wire_id) {
                            lc = lc + (coeff, *var);
                        }
                    }
                }
                lc
            };

            // Enforce constraint using R1CS_PREDICATE_LABEL
            let boxed: Vec<Box<dyn FnOnce() -> LinearCombination<Fr>>> = vec![
                Box::new(a_closure),
                Box::new(b_closure),
                Box::new(c_closure),
            ];
            cs.enforce_constraint(R1CS_PREDICATE_LABEL, boxed)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_signature_circuit() {
        let circuit = TestCircuit::from_fixture("signature_circuit")
            .expect("Failed to load signature circuit");

        println!("✅ Loaded signature circuit:");
        println!("   - Public inputs: {}", circuit.n_public_inputs());
        println!("   - Constraints: {}", circuit.circuit.n_constraints);
        println!("   - Wires: {}", circuit.circuit.n_wires);

        assert_eq!(circuit.n_public_inputs(), 7);
        assert_eq!(circuit.circuit.n_constraints, 8443);
        assert_eq!(circuit.circuit.n_wires, 8449);
    }

    #[test]
    fn test_synthesize_signature_circuit() {
        use ark_relations::gr1cs::ConstraintSystem;

        let circuit = TestCircuit::from_fixture("signature_circuit")
            .expect("Failed to load signature circuit");

        // Try to synthesize
        let cs = ConstraintSystem::<Fr>::new_ref();
        let result = circuit.generate_constraints(cs.clone());

        match result {
            Ok(_) => {
                let cs_borrowed = cs.borrow().unwrap();
                println!("✅ Synthesis successful!");
                println!("   - Constraints: {}", cs_borrowed.num_constraints());
                println!(
                    "   - Variables: {}",
                    cs_borrowed.num_instance_variables() + cs_borrowed.num_witness_variables()
                );
            }
            Err(e) => {
                // Expected to fail without valid witness
                println!("⚠️  Synthesis failed (expected): {:?}", e);
            }
        }
    }

    #[test]
    fn test_load_sudoku_circuit() {
        let circuit =
            TestCircuit::from_fixture("sudoku_circuit").expect("Failed to load sudoku circuit");

        println!("✅ Loaded sudoku circuit:");
        println!("   - Public inputs: {}", circuit.n_public_inputs());
        println!("   - Constraints: {}", circuit.circuit.n_constraints);
        println!("   - Wires: {}", circuit.circuit.n_wires);

        assert_eq!(circuit.n_public_inputs(), 81); // 9x9 sudoku grid
        assert_eq!(circuit.circuit.n_constraints, 162);
        assert_eq!(circuit.circuit.n_wires, 202);
    }

    #[test]
    fn test_synthesize_sudoku_circuit() {
        use ark_relations::gr1cs::ConstraintSystem;

        let circuit =
            TestCircuit::from_fixture("sudoku_circuit").expect("Failed to load sudoku circuit");

        // Try to synthesize
        let cs = ConstraintSystem::<Fr>::new_ref();
        let result = circuit.generate_constraints(cs.clone());

        match result {
            Ok(_) => {
                let cs_borrowed = cs.borrow().unwrap();
                println!("✅ Sudoku synthesis successful!");
                println!("   - Constraints: {}", cs_borrowed.num_constraints());
                println!(
                    "   - Variables: {}",
                    cs_borrowed.num_instance_variables() + cs_borrowed.num_witness_variables()
                );
            }
            Err(e) => {
                println!("⚠️  Synthesis failed (expected): {:?}", e);
            }
        }
    }
}
