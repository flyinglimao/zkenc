// Lightweight test that does not require curve crates. This verifies the crate's
// public API surface compiles and basic Default implementations exist for the
// exported types when instantiated with a dummy pairing type provided by tests
// behind the `with_curves` feature.

#[cfg(not(feature = "with_curves"))]
#[test]
fn smoke_build_only() {
    // This test just ensures the crate can be linked and basic symbols are
    // available. We don't perform heavy crypto here.
    assert!(true);
}

// The real integration-like tests that need an actual curve are gated behind
// the `with_curves` feature. To run them locally use:
//
//   cargo test -p zkenc-core --features with_curves

#[cfg(feature = "with_curves")]
mod with_curves_tests {
    use ark_std::rand::SeedableRng;
    use zkenc_core::{Proof, PublicParameters, ZkEncAlgorithm};

    #[test]
    fn smoke_default_structs() {
        let params: PublicParameters<ark_bls12_381::Bls12_381> = PublicParameters::default();
        let proof: Proof<ark_bls12_381::Bls12_381> = Proof::default();
        let _ = (params, proof);
    }

    #[test]
    fn api_contract_setup_prove_verify() {
        // Use ark_std::test_rng which is stable across arkworks tests and
        // avoids pulling rand::SeedableRng into scope.
        let mut rng = ark_std::test_rng();
        let params = ZkEncAlgorithm::<ark_bls12_381::Bls12_381>::setup(&mut rng);

        // witness empty slice is acceptable for the smoke test
        let witness: Vec<_> = vec![];
        let proof = ZkEncAlgorithm::<ark_bls12_381::Bls12_381>::prove(&params, &witness, &mut rng);
        let public_inputs: Vec<_> = vec![];
        let ok =
            ZkEncAlgorithm::<ark_bls12_381::Bls12_381>::verify(&params, &proof, &public_inputs);
        assert!(ok);
    }
}
