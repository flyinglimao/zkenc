//! R1CS to QAP conversion helpers
//!
//! This module provides utilities to convert R1CS constraint systems to QAP.
//! Reference: Groth16's r1cs_to_qap implementation

use ark_ff::PrimeField;
use ark_poly::{EvaluationDomain, GeneralEvaluationDomain};
use ark_relations::gr1cs::ConstraintSystemRef;
use ark_std::vec::Vec;

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
/// This function is reserved for future full QAP conversion implementation.
#[allow(dead_code)]
pub fn create_domain<F: PrimeField>(num_constraints: usize) -> GeneralEvaluationDomain<F> {
    GeneralEvaluationDomain::<F>::new(num_constraints).expect("Failed to create evaluation domain")
}

/// Evaluate all QAP polynomials at point x
///
/// For each variable i, evaluates:
/// - u_i(x): Left constraint polynomial
/// - v_i(x): Right constraint polynomial  
/// - w_i(x): Output constraint polynomial
///
/// Returns (u_evals, v_evals, w_evals) where each is a Vec of evaluations
///
/// This uses the Lagrange interpolation approach from Groth16's LibsnarkReduction:
/// 1. Create evaluation domain of size num_constraints + num_instance_variables
/// 2. Evaluate all Lagrange basis polynomials L_j(x) at x
/// 3. For each variable i, compute uᵢ(x) = Σⱼ L_j(x) * A[j][i]
///    where A[j][i] is the coefficient of variable i in constraint j
pub fn evaluate_qap_polynomials_at_x<F: PrimeField>(
    cs: &ConstraintSystemRef<F>,
    x: F,
) -> (Vec<F>, Vec<F>, Vec<F>) {
    use ark_relations::gr1cs::R1CS_PREDICATE_LABEL;

    let m = num_variables(cs);

    // Get constraint matrices
    let matrices = match cs.to_matrices() {
        Ok(m) => m,
        Err(_) => {
            // If matrices unavailable, return zeros
            return (vec![F::zero(); m], vec![F::zero(); m], vec![F::zero(); m]);
        }
    };

    let constraint_matrices = &matrices[R1CS_PREDICATE_LABEL];
    if constraint_matrices.len() < 3 {
        // Invalid matrices, return zeros
        return (vec![F::zero(); m], vec![F::zero(); m], vec![F::zero(); m]);
    }

    // Create evaluation domain
    // Domain size = num_constraints + num_instance_variables
    let domain_size = num_constraints(cs) + num_public_inputs(cs);
    let domain = match GeneralEvaluationDomain::<F>::new(domain_size) {
        Some(d) => d,
        None => {
            // Domain too large, return zeros
            return (vec![F::zero(); m], vec![F::zero(); m], vec![F::zero(); m]);
        }
    };

    // Evaluate all Lagrange basis polynomials at x
    // L_j(x) is the unique polynomial that equals 1 at ω^j and 0 at all other domain points
    let lagrange_coeffs = domain.evaluate_all_lagrange_coefficients(x);

    // Initialize result vectors
    let mut u_evals = vec![F::zero(); m];
    let mut v_evals = vec![F::zero(); m];
    let mut w_evals = vec![F::zero(); m];

    // For each constraint j, accumulate L_j(x) * matrix[j][i] into result[i]
    // This computes uᵢ(x) = Σⱼ L_j(x) * A[j][i] for all variables i
    let n_constraints = num_constraints(cs);
    for (j, &lagrange_j) in lagrange_coeffs.iter().enumerate().take(n_constraints) {
        // Matrix A (corresponds to u polynomials)
        for &(ref coeff, index) in &constraint_matrices[0][j] {
            if index < m {
                u_evals[index] += lagrange_j * coeff;
            }
        }

        // Matrix B (corresponds to v polynomials)
        for &(ref coeff, index) in &constraint_matrices[1][j] {
            if index < m {
                v_evals[index] += lagrange_j * coeff;
            }
        }

        // Matrix C (corresponds to w polynomials)
        for &(ref coeff, index) in &constraint_matrices[2][j] {
            if index < m {
                w_evals[index] += lagrange_j * coeff;
            }
        }
    }

    (u_evals, v_evals, w_evals)
}
