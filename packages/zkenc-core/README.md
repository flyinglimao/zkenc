# zkenc-core# zkenc-core

**Witness Key Encapsulation Mechanism (WKEM) for Quadratic Arithmetic Programs (QAP)**這是 zkenc-core 專案，包含核心演算法的實作。該專案提供了主要的邏輯和功能，並可供其他子專案（如 zkenc-cli 和 zkenc-js）使用。

A cryptographic library implementing a witness encryption scheme based on QAP satisfiability. This library provides key encapsulation and decapsulation functions that bind encryption keys to the satisfiability of constraint systems.## 安裝

## Overview 要安裝該專案，請確保您已安裝 Rust 和 Cargo。然後，您可以使用以下命令來克隆該專案並編譯：

zkenc-core implements a WKEM scheme where:```bash

- **Encapsulation** generates a fresh CRS (Common Reference String) and derives a symmetric key from a circuit with only public inputs assignedgit clone <repository-url>

- **Decapsulation** recovers the same key by providing a valid witness that satisfies the circuit constraintscd zkenc-core

cargo build

The security relies on pairing-based cryptography over elliptic curves, similar to zkSNARK constructions like Groth16.```

## Installation## 使用

Add to your `Cargo.toml`:在成功編譯後，您可以在其他專案中將其作為依賴項使用，或直接在命令行中調用。

```toml## 貢獻

[dependencies]

zkenc-core = { path = "../zkenc-core", features = ["std"] }歡迎任何形式的貢獻！請查看貢獻指南以獲取更多信息。

# For testing with concrete curves
[dev-dependencies]
zkenc-core = { path = "../zkenc-core", features = ["with_curves"] }
```

## Features

- `std`: Standard library support (enabled by default)
- `r1cs`: R1CS constraint system support (required for circuit operations)
- `with_curves`: Enables BLS12-381 curve for testing and examples
- `parallel`: Parallel computation support (native only)

## Usage

### Basic Example

```rust
use zkenc_core::{encap, decap, Ciphertext, Key};
use ark_bls12_381::Bls12_381;
use ark_relations::gr1cs::ConstraintSynthesizer;
use ark_std::test_rng;

// Define your circuit implementing ConstraintSynthesizer<F>
struct MyCircuit { /* ... */ }

// Encapsulation: Generate ciphertext and key with public inputs only
let circuit_encap = MyCircuit::new(/* public inputs only */);
let mut rng = test_rng();
let (ciphertext, key1) = encap::<Bls12_381, _, _>(circuit_encap, &mut rng)?;

// Decapsulation: Recover key with full witness
let circuit_decap = MyCircuit::new(/* public inputs + witness */);
let key2 = decap::<Bls12_381, _>(circuit_decap, &ciphertext)?;

assert_eq!(key1, key2);
```

### API Reference

#### `encap<E, C, R>(circuit: C, rng: &mut R) -> Result<(Ciphertext<E>, Key), Error>`

Generate a ciphertext and derive a key from a circuit with public inputs.

**Type Parameters:**

- `E: Pairing` - Pairing-friendly elliptic curve (e.g., `Bls12_381`)
- `C: ConstraintSynthesizer<E::ScalarField>` - Circuit implementing constraint synthesis
- `R: RngCore` - Random number generator

**Parameters:**

- `circuit` - Circuit with public inputs assigned, witness unassigned
- `rng` - Cryptographically secure random number generator

**Returns:**

- `Ciphertext<E>` - Contains CRS σ and public inputs
- `Key` - 32-byte symmetric key derived from pairing computation

**Errors:**

- `Error::SynthesisError` - Circuit synthesis failed
- `Error::SerializationError` - Failed to serialize pairing result

**Example:**

```rust
use ark_std::rand::SeedableRng;

let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(42);
let circuit = MyCircuit::new(None, None, Some(public_output));
let (ciphertext, key) = encap::<Bls12_381, _, _>(circuit, &mut rng)?;
```

---

#### `decap<E, C>(circuit: C, ciphertext: &Ciphertext<E>) -> Result<Key, Error>`

Recover the key by providing a valid witness satisfying the circuit.

**Type Parameters:**

- `E: Pairing` - Pairing-friendly elliptic curve (must match encap)
- `C: ConstraintSynthesizer<E::ScalarField>` - Circuit with full assignment

**Parameters:**

- `circuit` - Circuit with both public inputs and witness assigned
- `ciphertext` - Ciphertext from `encap` containing CRS σ

**Returns:**

- `Key` - The recovered 32-byte symmetric key (same as from `encap`)

**Errors:**

- `Error::InvalidWitness` - Circuit constraints not satisfied
- `Error::InvalidPublicInputs` - Public inputs don't match ciphertext
- `Error::SynthesisError` - Circuit synthesis failed
- `Error::SerializationError` - Failed to serialize pairing result

**Example:**

```rust
let circuit = MyCircuit::new(Some(witness_l), Some(witness_r), Some(public_output));
let recovered_key = decap::<Bls12_381, _>(circuit, &ciphertext)?;
assert_eq!(original_key, recovered_key);
```

---

#### `verify_ciphertext<E>(ciphertext: &Ciphertext<E>, expected_public_inputs: &[E::ScalarField]) -> bool`

Verify that a ciphertext contains expected public inputs (utility function for debugging).

**Type Parameters:**

- `E: Pairing` - Pairing-friendly elliptic curve

**Parameters:**

- `ciphertext` - Ciphertext to verify
- `expected_public_inputs` - Expected public input values

**Returns:**

- `bool` - `true` if public inputs match, `false` otherwise

---

### Data Structures

#### `Ciphertext<E: Pairing>`

Contains the encapsulation key (CRS) and public inputs.

**Fields:**

- `encap_key: EncapKey<E>` - The CRS σ generated during encapsulation
- `public_inputs: Vec<E::ScalarField>` - Public input values (excluding constant 1)

**Traits:**

- `CanonicalSerialize`, `CanonicalDeserialize` - For serialization
- `Clone`, `Debug`, `PartialEq`

---

#### `EncapKey<E: Pairing>`

The Common Reference String (CRS) σ containing group elements.

**Fields:**

- `alpha_g1: E::G1Affine` - [α]₁ in G1
- `beta_g2: E::G2Affine` - [β]₂ in G2
- `delta_g2: E::G2Affine` - [δ]₂ in G2
- `r_u_query_g1: Vec<E::G1Affine>` - [r·uᵢ(x)]₁ for each variable
- `r_v_query_g2: Vec<E::G2Affine>` - [r·vᵢ(x)]₂ for each variable
- `phi_delta_query_g1: Vec<E::G1Affine>` - [φᵢ(x)/δ]₁ where φᵢ(x) = r·β·uᵢ(x) + r·α·vᵢ(x) + r²·wᵢ(x)
- `h_query_g1: Vec<E::G1Affine>` - Quotient polynomial query (currently unused)

---

#### `Key`

A 32-byte symmetric key derived from pairing computation.

**Constructors:**

- `Key::new(bytes: [u8; 32])` - Create from byte array
- `Key::default()` - Zero-initialized key

**Methods:**

- `as_bytes(&self) -> &[u8; 32]` - Access raw bytes
- `to_hex(&self) -> String` - Convert to hex string (requires `std`)

**Traits:**

- `PartialEq`, `Eq`, `Clone`, `Debug`, `Zeroize` (drops securely)

---

### Error Handling

```rust
pub enum Error {
    SynthesisError(String),     // Circuit synthesis failed
    InvalidWitness,              // Circuit not satisfied
    InvalidPublicInputs,         // Public inputs mismatch
    SerializationError,          // Serialization failed
}
```

All functions return `Result<T, Error>`. Use standard Rust error handling:

```rust
match encap(circuit, &mut rng) {
    Ok((ciphertext, key)) => { /* success */ },
    Err(Error::SynthesisError(msg)) => eprintln!("Synthesis failed: {}", msg),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Architecture

### Algorithm Flow

**Encapsulation:**

1. Synthesize circuit with public inputs only
2. Sample random parameters (α, β, δ, r, x)
3. Evaluate QAP polynomials uᵢ(x), vᵢ(x), wᵢ(x)
4. Generate CRS query vectors with MSM
5. Compute pairing s = e([α]₁, [β]₂) + e(Σ aᵢ·[φᵢ]₁, [1]₂)
6. Derive key via serialization and hashing

**Decapsulation:**

1. Synthesize circuit with full assignment (public + witness)
2. Verify circuit is satisfied
3. Compute A = [α]₁ + Σ aᵢ·[r·uᵢ(x)]₁
4. Compute B = [β]₂ + Σ aᵢ·[r·vᵢ(x)]₂
5. Compute C = Σ aᵢ·[φᵢ(x)/δ]₁
6. Compute pairing s = e(A, B) - e(C, [δ]₂)
7. Derive key using same KDF as encapsulation

### Cryptographic Primitives

- **Elliptic Curves**: Via `ark-ec` (BLS12-381 recommended)
- **Finite Fields**: Via `ark-ff`
- **Constraint Systems**: Via `ark-relations` (R1CS/ConstraintSynthesizer)
- **Pairing Operations**: Via `ark-ec::pairing::Pairing`
- **Multi-Scalar Multiplication**: Via `ark-ec::VariableBaseMSM`

## Testing

Run the full test suite:

```bash
# All tests (requires curve feature)
cargo test -p zkenc-core --features with_curves

# Specific test
cargo test -p zkenc-core --features with_curves test_encap_decap_correctness

# With output
cargo test -p zkenc-core --features with_curves -- --nocapture
```

### Test Coverage

The library includes comprehensive integration tests using a MiMC-322 hash circuit:

- ✅ `test_encap_decap_correctness` - Basic encap/decap flow
- ✅ `test_encap_decap_wrong_witness` - Invalid witness rejection
- ✅ `test_encap_different_public_inputs` - Different randomness
- ✅ `test_ciphertext_serialization` - Serialization round-trip
- ✅ MiMC circuit unit tests

## Limitations and Future Work

### Current Limitations

1. **Placeholder QAP Conversion**: The R1CS to QAP polynomial evaluation currently returns zero vectors. Full FFT/IFFT-based polynomial interpolation is reserved for future implementation.

2. **Key Derivation**: Uses simple serialization truncation. Production systems should use proper KDF (HKDF/Blake3).

3. **Circuit Synthesis**: Encapsulation with missing witness shows 0 constraints. This is expected behavior but could be handled more gracefully.

### Planned Improvements

- Complete FFT/IFFT-based R1CS to QAP conversion
- Proper cryptographic KDF implementation
- Performance optimization (parallel MSM, batch pairing)
- Additional curve support (BN254, BW6-761)
- Comprehensive security audit

## Documentation

- **DESIGN.md** - Detailed mathematical construction and comparison with Groth16
- **TEST_PLAN.md** - TDD strategy and test case documentation
- **API Documentation** - Run `cargo doc --open -p zkenc-core --features with_curves`

## Contributing

Contributions are welcome! Please ensure:

- All tests pass: `cargo test -p zkenc-core --features with_curves`
- Code is formatted: `cargo fmt -p zkenc-core`
- No warnings: `cargo clippy -p zkenc-core`

## License

This project is dual-licensed under MIT/Apache-2.0.

## References

- **WKEM Paper**: [Original witness encryption paper/scheme reference]
- **Groth16**: "On the Size of Pairing-based Non-interactive Arguments" by Jens Groth
- **arkworks**: https://github.com/arkworks-rs
