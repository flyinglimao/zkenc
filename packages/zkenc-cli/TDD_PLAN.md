# zkenc-cli TDD Plan

## Phase 1: Basic Circom Loading (Test-First)
**Goal**: Load and parse circom R1CS and WASM files

### Test 1.1: Load circom files
- [ ] Test: `test_load_circom_circuit`
- [ ] Load signature.r1cs and signature.wasm
- [ ] Verify circuit info (num constraints, variables)

### Implementation 1.1: Circom module
- [ ] Add circom-compat dependency
- [ ] Create `circom.rs` module
- [ ] Implement `load_circom_circuit()`

---

## Phase 2: Input Parsing (Test-First)
**Goal**: Parse JSON input files to field elements

### Test 2.1: Parse simple inputs
- [ ] Test: `test_parse_simple_input`
- [ ] Parse signature_basic.json
- [ ] Verify field element conversion

### Implementation 2.1: Input parsing
- [ ] Implement `parse_inputs()`
- [ ] Handle strings, numbers, arrays

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
