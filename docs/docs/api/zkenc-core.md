---
sidebar_position: 1
---

# zkenc-core API Reference

zkenc-core is the Rust library that provides the cryptographic primitives for witness encryption. It serves as the foundation layer - both zkenc-cli and zkenc-js are built on top of this core library.

## Overview

zkenc-core implements witness encryption for R1CS circuits using elliptic curve cryptography (BN254 curve). It provides two core functions:

- **`encap`**: Generate a witness-encrypted key
- **`decap`**: Recover the key using a valid witness

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
zkenc-core = { path = "../zkenc-core" }
ark-bn254 = "0.4"
ark-std = "0.4"
ark-serialize = "0.4"
```

## Core Types

### `Ciphertext<E: Pairing>`

Represents the witness encryption ciphertext.

```rust
pub struct Ciphertext<E: Pairing> {
    // Internal fields are private
}
```

**Properties:**

- Serializable using arkworks serialization
- Size: ~1576 bytes for BN254 curve
- Can be sent over network or stored to disk

### `Key`

Represents the symmetric encryption key.

```rust
pub struct Key {
    // Internal: 32-byte key
}
```

**Methods:**

- `as_bytes() -> &[u8; 32]` - Get key as bytes for AES encryption

## Core Functions

### `encap`

Generate witness-encrypted ciphertext and key from a circuit.

```rust
pub fn encap<E, C, R>(
    circuit: C,
    rng: &mut R,
) -> Result<(Ciphertext<E>, Key), EncapError>
where
    E: Pairing,
    C: ConstraintSynthesizer<E::ScalarField>,
    R: RngCore + CryptoRng,
```

**Type Parameters:**

- `E` - Elliptic curve pairing (typically `Bn254`)
- `C` - Circuit implementing `ConstraintSynthesizer`
- `R` - Random number generator

**Parameters:**

- `circuit` - Circuit instance with public inputs set
- `rng` - Cryptographically secure random number generator

**Returns:**

- `Ok((Ciphertext, Key))` - Ciphertext and generated key
- `Err(EncapError)` - If circuit synthesis fails

**Example:**

```rust
use zkenc_core::{encap, Ciphertext, Key};
use ark_bn254::Bn254;
use ark_std::rand::rngs::OsRng;

// Create circuit with public inputs
let mut circuit = MyCircuit::new();
circuit.set_public_input(42);

// Generate ciphertext and key
let mut rng = OsRng;
let (ciphertext, key) = encap::<Bn254, _, _>(circuit, &mut rng)?;

// Serialize for storage/transmission
let mut ct_bytes = Vec::new();
ciphertext.serialize_compressed(&mut ct_bytes)?;
```

**Performance:**

- Time complexity: O(n \* log n) where n = number of constraints
- Memory: O(n) for constraint system
- Typical time: 50-500ms depending on circuit size

### `decap`

Recover the encryption key using a valid witness.

```rust
pub fn decap<E, C>(
    circuit: C,
    ciphertext: &Ciphertext<E>,
) -> Result<Key, DecapError>
where
    E: Pairing,
    C: ConstraintSynthesizer<E::ScalarField>,
```

**Type Parameters:**

- `E` - Elliptic curve pairing (typically `Bn254`)
- `C` - Circuit implementing `ConstraintSynthesizer`

**Parameters:**

- `circuit` - Circuit instance with full witness set
- `ciphertext` - Ciphertext from `encap`

**Returns:**

- `Ok(Key)` - Recovered encryption key
- `Err(DecapError)` - If witness is invalid or doesn't satisfy constraints

**Example:**

```rust
use zkenc_core::{decap, Ciphertext, Key};
use ark_bn254::Bn254;
use ark_serialize::CanonicalDeserialize;

// Load ciphertext
let ct_bytes = std::fs::read("ciphertext.bin")?;
let ciphertext = Ciphertext::<Bn254>::deserialize_compressed(&ct_bytes[..])?;

// Create circuit with full witness
let mut circuit = MyCircuit::new();
circuit.set_public_input(42);
circuit.set_private_input(123); // The witness

// Recover key
let key = decap::<Bn254, _>(circuit, &ciphertext)?;

// Use key for decryption
let key_bytes = key.as_bytes();
```

**Performance:**

- Time complexity: O(n) where n = number of constraints
- Memory: O(n) for constraint system
- Typical time: 100-1000ms depending on circuit size

## Error Types

### `EncapError`

Errors that can occur during encapsulation.

```rust
pub enum EncapError {
    /// Circuit synthesis failed
    SynthesisError(SynthesisError),
    /// Random number generation failed
    RngError,
}
```

### `DecapError`

Errors that can occur during decapsulation.

```rust
pub enum DecapError {
    /// Circuit synthesis failed
    SynthesisError(SynthesisError),
    /// Witness doesn't satisfy constraints
    InvalidWitness,
    /// Pairing check failed
    PairingCheckFailed,
}
```

## Circuit Interface

To use zkenc-core, your circuit must implement `ConstraintSynthesizer`:

```rust
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};

pub struct MyCircuit<F: Field> {
    pub public_input: Option<F>,
    pub private_input: Option<F>,
}

impl<F: Field> ConstraintSynthesizer<F> for MyCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        // Allocate public input
        let pub_var = cs.new_input_variable(|| {
            self.public_input.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Allocate private input (witness)
        let priv_var = cs.new_witness_variable(|| {
            self.private_input.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Add constraints
        // ... your circuit logic

        Ok(())
    }
}
```

## Complete Example

Here's a complete example of encryption and decryption:

```rust
use zkenc_core::{encap, decap};
use ark_bn254::Bn254;
use ark_ff::Field;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_r1cs_std::prelude::*;
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};
use ark_std::rand::rngs::OsRng;

// Define circuit
#[derive(Clone)]
struct SimpleCircuit {
    pub_input: Option<u64>,
    priv_input: Option<u64>,
}

impl ConstraintSynthesizer<ark_bn254::Fr> for SimpleCircuit {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ark_bn254::Fr>,
    ) -> Result<(), SynthesisError> {
        use ark_bn254::Fr;

        // Public input
        let pub_var = cs.new_input_variable(|| {
            self.pub_input.map(Fr::from).ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Private witness
        let priv_var = cs.new_witness_variable(|| {
            self.priv_input.map(Fr::from).ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Constraint: pub + priv = constant
        cs.enforce_constraint(
            lc!() + pub_var + priv_var,
            lc!() + Variable::One,
            lc!() + (165u64, Variable::One), // pub + priv = 165
        )?;

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ENCRYPTION
    println!("Encrypting...");

    // Create circuit with only public input
    let encrypt_circuit = SimpleCircuit {
        pub_input: Some(42),
        priv_input: None, // Not needed for encryption
    };

    // Encapsulate
    let mut rng = OsRng;
    let (ciphertext, key) = encap::<Bn254, _, _>(encrypt_circuit, &mut rng)?;

    println!("Generated key: {:?}", key.as_bytes());

    // Serialize ciphertext
    let mut ct_bytes = Vec::new();
    ciphertext.serialize_compressed(&mut ct_bytes)?;
    std::fs::write("ciphertext.bin", &ct_bytes)?;

    // Use key for AES encryption
    let message = b"Secret message";
    // ... encrypt message with key.as_bytes() ...

    // DECRYPTION
    println!("\nDecrypting...");

    // Load ciphertext
    let ct_bytes = std::fs::read("ciphertext.bin")?;
    let ciphertext = Ciphertext::<Bn254>::deserialize_compressed(&ct_bytes[..])?;

    // Create circuit with full witness
    let decrypt_circuit = SimpleCircuit {
        pub_input: Some(42),
        priv_input: Some(123), // Must satisfy: 42 + 123 = 165
    };

    // Decapsulate
    let recovered_key = decap::<Bn254, _>(decrypt_circuit, &ciphertext)?;

    println!("Recovered key: {:?}", recovered_key.as_bytes());

    // Verify keys match
    assert_eq!(key.as_bytes(), recovered_key.as_bytes());
    println!("✅ Keys match!");

    Ok(())
}
```

## Integration with Other Tools

### With zkenc-cli

zkenc-cli uses zkenc-core internally:

```rust
use zkenc_core::{encap, decap};
use zkenc_cli::circuit::CircomCircuit;

// Load R1CS circuit
let circuit = CircomCircuit::from_r1cs("circuit.r1cs")?;

// Encap
let (ciphertext, key) = encap::<Bn254, _, _>(circuit, &mut rng)?;
```

### With Custom Circuits

```rust
// Your custom circuit
impl ConstraintSynthesizer<Fr> for MyCircuit {
    fn generate_constraints(/* ... */) -> Result<(), SynthesisError> {
        // Your constraints
    }
}

// Use with zkenc-core
let circuit = MyCircuit::new(/* params */);
let (ct, key) = encap::<Bn254, _, _>(circuit, &mut rng)?;
```

## Performance Tips

1. **Circuit Optimization**: Minimize constraints for faster operations
2. **Batch Operations**: Reuse circuit compilation when possible
3. **Memory Management**: Use compressed serialization to reduce size
4. **Parallel Processing**: encap/decap can be parallelized across multiple messages

## Curve Selection

Currently, zkenc-core uses BN254 curve by default:

```rust
use ark_bn254::Bn254;

let (ct, key) = encap::<Bn254, _, _>(circuit, &mut rng)?;
```

Future versions may support additional curves (BLS12-381, etc.).

## Security Considerations

1. **RNG Security**: Always use cryptographically secure RNG (`OsRng`)
2. **Witness Privacy**: Never expose private witness values
3. **Circuit Correctness**: Ensure circuit properly enforces constraints
4. **Key Usage**: Use keys with proper symmetric encryption (AES-256-GCM)

## Next Steps

- **[zkenc-cli API →](/docs/api/zkenc-cli)** - Command-line interface
- **[zkenc-js API →](/docs/api/zkenc-js)** - JavaScript bindings
- **[Getting Started →](/docs/getting-started/zkenc-cli)** - Quick start guide
