# zkenc-cli TDD Plan

## Phase 1: Basic Circom Loading (Test-First) ✅

**Goal**: Load and parse circom R1CS and WASM files

### Test 1.1: Load circom files ✅

- [x] Test: `test_load_circom_circuit`
- [x] Load signature.r1cs and signature.wasm
- [x] Verify circuit info (num constraints, variables)
- **Result**: 2294 constraints, 3 public inputs, 4588 variables

### Implementation 1.1: Circom module ✅

- [x] Add lib.rs for testing support
- [x] Create `circom.rs` module
- [x] Implement `load_circom_circuit()` - minimal version
- Note: Full ark-circom integration deferred to Phase 3

---

## Phase 2: Input Parsing (Test-First) ✅

**Goal**: Parse JSON input files to field elements

### Test 2.1: Parse simple inputs ✅

- [x] Test: `test_parse_simple_input`
- [x] Parse signature_basic.json
- [x] Verify field element conversion
- **Result**: Successfully parsed 5 fields (message, publicKeys, R8, S, signerIndices)

### Implementation 2.1: Input parsing ✅

- [x] Implement `parse_inputs()` - returns HashMap<String, Vec<String>>
- [x] Handle strings, numbers, arrays
- [x] Recursive flattening of nested arrays
- Note: Flattens 2D arrays (e.g., publicKeys: 3×2 → 6 values)

---

## Phase 3: Encap/Decap Integration (Test-First)

**Goal**: Integrate with zkenc-core

### Test 3.1: Encap with circom circuit

- [ ] Test: `test_encap_with_circom`
- [ ] Load circuit with public inputs only
- [ ] Call zkenc-core encap
- [ ] Verify key generation

### Test 3.2: Decap with circom circuit

- [ ] Test: `test_decap_with_circom`
- [ ] Load circuit with full witness
- [ ] Call zkenc-core decap
- [ ] Verify key recovery

### Implementation 3.1: Circuit wrapper

- [ ] Create CircomCircuitWrapper
- [ ] Implement ConstraintSynthesizer

---

## Phase 4: AES Encryption (Test-First)

**Goal**: Encrypt/decrypt with derived key

### Test 4.1: AES-GCM roundtrip

- [ ] Test: `test_aes_gcm_roundtrip`
- [ ] Encrypt plaintext
- [ ] Decrypt and verify

### Test 4.2: AES-CTR roundtrip

- [ ] Test: `test_aes_ctr_roundtrip`

### Implementation 4.1: Crypto module

- [ ] Implement `encrypt()` and `decrypt()`
- [ ] Support GCM and CTR modes

---

## Phase 5: CLI Commands (Test-First)

**Goal**: Complete CLI integration

### Test 5.1: Encap command

- [ ] Test: End-to-end encap with file I/O

### Test 5.2: Decap command

- [ ] Test: End-to-end decap with file I/O

### Test 5.3: Encrypt/Decrypt commands

- [ ] Test: Full encrypt/decrypt flow

### Implementation 5.1: Commands module

- [ ] Implement all 4 commands
- [ ] File I/O handling

---

## Progress Tracker

- Phase 1: ⏳ Starting
- Phase 2: ⏸️ Not started
- Phase 3: ⏸️ Not started
- Phase 4: ⏸️ Not started
- Phase 5: ⏸️ Not started
