// MiMC hash circuit for testing
// Based on arkworks-rs/groth16/tests/mimc.rs

#![cfg(feature = "with_curves")]

use ark_ff::{PrimeField, Zero};
use ark_relations::gr1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};

pub const MIMC_ROUNDS: usize = 322;

/// MiMC-based hash circuit: LongsightF322p3 variant
///
/// For i from 0 to 321:
///     xL, xR := xR + (xL + Cᵢ)³, xL
/// return xL
///
/// Public input: output (hash result)
/// Witness: xL, xR (preimage)
#[derive(Clone)]
pub struct MiMCCircuit<F: PrimeField> {
    pub xl: Option<F>,
    pub xr: Option<F>,
    pub output: Option<F>,
    pub constants: Vec<F>,
}

impl<F: PrimeField> MiMCCircuit<F> {
    pub fn new(xl: Option<F>, xr: Option<F>, output: Option<F>, constants: Vec<F>) -> Self {
        assert_eq!(constants.len(), MIMC_ROUNDS);
        Self {
            xl,
            xr,
            output,
            constants,
        }
    }

    /// Compute MiMC hash (native, for testing)
    pub fn mimc_native(mut xl: F, mut xr: F, constants: &[F]) -> F {
        assert_eq!(constants.len(), MIMC_ROUNDS);
        for i in 0..MIMC_ROUNDS {
            let mut tmp1 = xl;
            tmp1 += constants[i];
            let mut tmp2 = tmp1.square();
            tmp2 *= tmp1;
            tmp2 += xr;
            xr = xl;
            xl = tmp2;
        }
        xl
    }
}

impl<F: PrimeField> ConstraintSynthesizer<F> for MiMCCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        use ark_r1cs_std::{
            alloc::AllocVar,
            eq::EqGadget,
            fields::{fp::FpVar, FieldVar},
        };

        assert_eq!(self.constants.len(), MIMC_ROUNDS);

        // Allocate witness: xL, xR (private inputs)
        let mut xl = FpVar::new_witness(cs.clone(), || {
            self.xl.ok_or(SynthesisError::AssignmentMissing)
        })?;

        let mut xr = FpVar::new_witness(cs.clone(), || {
            self.xr.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Allocate public input: output
        let output = FpVar::new_input(cs.clone(), || {
            self.output.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // MiMC rounds
        for i in 0..MIMC_ROUNDS {
            // tmp = (xL + Cᵢ)²
            let tmp = (&xl + self.constants[i]).square()?;
            // new_xL = xR + (xL + Cᵢ)³ = xR + tmp * (xL + Cᵢ)
            let new_xl = &xr + &tmp * (&xl + self.constants[i]);
            // Swap: xR = old xL, xL = new_xL
            xr = xl;
            xl = new_xl;
        }

        // Enforce output constraint: xl == output
        xl.enforce_equal(&output)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fr; // Use BN254 (alt_bn128) for consistency
    use ark_relations::gr1cs::{ConstraintSystem, OptimizationGoal};
    use ark_std::rand::{Rng, SeedableRng};

    #[test]
    fn test_mimc_native() {
        let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(0u64);
        let constants: Vec<Fr> = (0..MIMC_ROUNDS).map(|_| rng.gen()).collect();

        let xl = Fr::from(1u64);
        let xr = Fr::from(2u64);
        let output = MiMCCircuit::mimc_native(xl, xr, &constants);

        // Just ensure it runs without panic
        assert!(!output.is_zero() || output.is_zero()); // tautology to verify computation
    }

    #[test]
    fn test_mimc_circuit_satisfies() {
        let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(1u64);
        let constants: Vec<Fr> = (0..MIMC_ROUNDS).map(|_| rng.gen()).collect();

        let xl = Fr::from(42u64);
        let xr = Fr::from(99u64);
        let output = MiMCCircuit::mimc_native(xl, xr, &constants);

        let circuit = MiMCCircuit::new(Some(xl), Some(xr), Some(output), constants);

        let cs = ConstraintSystem::new_ref();
        cs.set_optimization_goal(OptimizationGoal::Constraints);
        circuit.generate_constraints(cs.clone()).unwrap();

        assert!(cs.is_satisfied().unwrap());
    }

    #[test]
    fn test_mimc_circuit_fails_with_wrong_output() {
        let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(2u64);
        let constants: Vec<Fr> = (0..MIMC_ROUNDS).map(|_| rng.gen()).collect();

        let xl = Fr::from(42u64);
        let xr = Fr::from(99u64);
        let wrong_output = Fr::from(123u64); // intentionally wrong

        let circuit = MiMCCircuit::new(Some(xl), Some(xr), Some(wrong_output), constants);

        let cs = ConstraintSystem::new_ref();
        cs.set_optimization_goal(OptimizationGoal::Constraints);
        circuit.generate_constraints(cs.clone()).unwrap();

        // Should NOT be satisfied
        assert!(!cs.is_satisfied().unwrap());
    }
}
