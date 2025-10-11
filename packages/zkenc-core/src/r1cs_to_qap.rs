//! R1CS to QAP conversion helpers
//!
//! This module provides utilities to convert R1CS constraint systems to QAP.
//! Reference: Groth16's r1cs_to_qap implementation

use ark_ff::PrimeField;
use ark_poly::{EvaluationDomain, GeneralEvaluationDomain};
use ark_relations::gr1cs::ConstraintSystemRef;

/// Get the number of public inputs (ℓ in the scheme)
pub fn num_public_inputs<F: PrimeField>(cs: &ConstraintSystemRef<F>) -> usize {
    cs.num_instance_variables()
}

/// Get the number of witness variables
pub fn num_witness_variables<F: PrimeField>(cs: &ConstraintSystemRef<F>) -> usize {
    cs.num_witness_variables()
}

/// Get total number of variables (m in the scheme)
pub fn num_variables<F: PrimeField>(cs: &ConstraintSystemRef<F>) -> usize {
    num_public_inputs(cs) + num_witness_variables(cs)
}

/// Get the number of constraints (n in the scheme)
pub fn num_constraints<F: PrimeField>(cs: &ConstraintSystemRef<F>) -> usize {
    cs.num_constraints()
}

/// Create an evaluation domain for QAP
///
/// The domain size must be at least the number of constraints.
pub fn create_domain<F: PrimeField>(num_constraints: usize) -> GeneralEvaluationDomain<F> {
    GeneralEvaluationDomain::<F>::new(num_constraints)
        .expect("Failed to create evaluation domain")
}
