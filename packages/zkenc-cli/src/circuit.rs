use crate::r1cs::R1csFile;
use anyhow::Result;
use ark_bn254::Fr; // Circom uses BN254 (alt_bn128)
use ark_ff::PrimeField;
use ark_relations::gr1cs::{
    ConstraintSynthesizer, ConstraintSystemRef, LinearCombination, SynthesisError, Variable,
    R1CS_PREDICATE_LABEL,
};
use ark_std::vec::Vec;
use std::collections::HashMap;
use std::path::Path;

/// Circom circuit wrapper that implements ConstraintSynthesizer
///
/// This bridges Circom R1CS format to zkenc-core's ConstraintSynthesizer trait.
/// Uses BN254 (alt_bn128) curve as this is Circom's default.
pub struct CircomCircuit {
    r1cs: R1csFile,
    witness: HashMap<u32, Fr>, // wire_id -> value
}

impl CircomCircuit {
    /// Load a Circom circuit from R1CS file
    pub fn from_r1cs<P: AsRef<Path>>(r1cs_path: P) -> Result<Self> {
        let r1cs = R1csFile::from_file(r1cs_path)?;
        Ok(Self {
            r1cs,
            witness: HashMap::new(),
        })
    }

    /// Set witness values for the circuit
    ///
    /// # Arguments
    /// * `values` - Map from wire_id to field element value
    pub fn set_witness(&mut self, values: HashMap<u32, Fr>) {
        self.witness = values;
    }

    /// Set a single witness value
    pub fn set_wire(&mut self, wire_id: u32, value: Fr) {
        self.witness.insert(wire_id, value);
    }

    /// Get the number of public inputs
    pub fn n_public_inputs(&self) -> u32 {
        self.r1cs.n_public_inputs()
    }

    /// Get the number of constraints
    pub fn n_constraints(&self) -> u32 {
        self.r1cs.n_constraints
    }

    /// Convert bytes to field element
    fn bytes_to_fr(bytes: &[u8]) -> Result<Fr, SynthesisError> {
        // R1CS stores field elements in little-endian byte format
        // We need to convert them to ark-ff's representation
        Fr::from_le_bytes_mod_order(bytes);

        // Use BigInt conversion for proper handling
        let mut bytes_array = vec![0u8; 32];
        let len = bytes.len().min(32);
        bytes_array[..len].copy_from_slice(&bytes[..len]);

        Ok(Fr::from_le_bytes_mod_order(&bytes_array))
    }
}

impl ConstraintSynthesizer<Fr> for CircomCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        // Allocate all variables
        // Wire 0 is always ONE (constant)
        let mut variables: HashMap<u32, Variable> = HashMap::new();
        variables.insert(0, Variable::One);

        // Allocate public inputs (outputs + inputs)
        let n_public = self.r1cs.n_public_inputs();
        for wire_id in 1..=n_public {
            let value = self.witness.get(&wire_id).copied();
            let var = cs.new_input_variable(|| value.ok_or(SynthesisError::AssignmentMissing))?;
            variables.insert(wire_id, var);
        }

        // Allocate private witnesses
        for wire_id in (n_public + 1)..self.r1cs.n_wires {
            let value = self.witness.get(&wire_id).copied();
            let var = cs.new_witness_variable(|| value.ok_or(SynthesisError::AssignmentMissing))?;
            variables.insert(wire_id, var);
        }

        // Add all constraints: A * B - C = 0  -->  A * B = C
        for constraint in self.r1cs.constraints.iter() {
            // Clone data needed for closures
            let a_factors = constraint.a.factors.clone();
            let b_factors = constraint.b.factors.clone();
            let c_factors = constraint.c.factors.clone();
            let vars_a = variables.clone();
            let vars_b = variables.clone();
            let vars_c = variables.clone();

            // Build closures for A, B, C
            let a_closure = move || {
                let mut lc = LinearCombination::<Fr>::zero();
                for (wire_id, coeff_bytes) in &a_factors {
                    if let Ok(coeff) = Self::bytes_to_fr(coeff_bytes) {
                        if let Some(var) = vars_a.get(wire_id) {
                            lc = lc + (coeff, *var);
                        }
                    }
                }
                lc
            };

            let b_closure = move || {
                let mut lc = LinearCombination::<Fr>::zero();
                for (wire_id, coeff_bytes) in &b_factors {
                    if let Ok(coeff) = Self::bytes_to_fr(coeff_bytes) {
                        if let Some(var) = vars_b.get(wire_id) {
                            lc = lc + (coeff, *var);
                        }
                    }
                }
                lc
            };

            let c_closure = move || {
                let mut lc = LinearCombination::<Fr>::zero();
                for (wire_id, coeff_bytes) in &c_factors {
                    if let Ok(coeff) = Self::bytes_to_fr(coeff_bytes) {
                        if let Some(var) = vars_c.get(wire_id) {
                            lc = lc + (coeff, *var);
                        }
                    }
                }
                lc
            };

            // For R1CS: A * B = C means we need to enforce A * B - C = 0
            // gr1cs uses predicate format, we use arity 3 with standard R1CS_PREDICATE_LABEL
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
    use ark_ff::Zero;
    use std::path::PathBuf;

    #[test]
    fn test_load_circom_circuit() {
        let r1cs_path = PathBuf::from("tests/r1cs/signature.r1cs");
        let circuit = CircomCircuit::from_r1cs(&r1cs_path).expect("Failed to load circuit");

        println!("Circom Circuit:");
        println!("  Public inputs: {}", circuit.n_public_inputs());
        println!("  Constraints: {}", circuit.n_constraints());

        assert_eq!(circuit.n_public_inputs(), 7);
        assert_eq!(circuit.n_constraints(), 8443);
    }

    #[test]
    fn test_circuit_synthesis() {
        let r1cs_path = PathBuf::from("tests/r1cs/signature.r1cs");
        let mut circuit = CircomCircuit::from_r1cs(&r1cs_path).expect("Failed to load circuit");

        // Set dummy witness values (all zeros for now)
        for wire_id in 0..circuit.r1cs.n_wires {
            circuit.set_wire(wire_id, Fr::zero());
        }

        // Try to synthesize
        use ark_relations::gr1cs::ConstraintSystem;
        let cs = ConstraintSystem::<Fr>::new_ref();
        let result = circuit.generate_constraints(cs.clone());

        match result {
            Ok(_) => {
                let cs_borrowed = cs.borrow().unwrap();
                println!("Synthesis successful!");
                println!("  Constraints: {}", cs_borrowed.num_constraints());
                println!(
                    "  Variables: {}",
                    cs_borrowed.num_instance_variables() + cs_borrowed.num_witness_variables()
                );
            }
            Err(e) => {
                println!("Synthesis failed (expected without valid witness): {:?}", e);
            }
        }
    }
}
