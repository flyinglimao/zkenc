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

## Phase 3: Encap/Decap Integration (Test-First) ⚠️ Blocked

**Goal**: Integrate with zkenc-core using ark-circom

**Status**: ⚠️ **Blocked** - ark-circom internal dependency conflict

### Blocker: ark-circom Mixed Dependency Sources

**Root Cause**: ark-circom (git version) has conflicting internal dependencies:
- `dependencies`: Uses crates.io ark-bn254 0.5.0
- `dev-dependencies`: Uses git versions
- **Problem**: crates.io ark-bn254 0.5.0 is incompatible with git ark-ec (missing `ZeroFlag` trait)

**Compilation Error**:
```
error[E0046]: not all trait items implemented, missing: `ZeroFlag`
 --> ark-bn254-0.5.0/src/curves/g1.rs:27:1
27 | impl SWCurveConfig for Config {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `ZeroFlag` in implementation
```

### Architecture Design ✅

- [x] **Design**: CircomCircuitWrapper structure
- [x] **Documentation**: Integration plan with code examples
- [x] **API**: Defined how to bridge Circom ↔ zkenc-core
- See `src/circom.rs` for detailed integration architecture

### Resolution Options

1. **Option A**: Fork ark-circom and change all dependencies to git versions
2. **Option B**: Wait for arkworks 0.5 stable crates.io release
3. **Option C**: Separate architecture - Node.js for Circom, Rust for zkenc-core

### Test 3.1: Encap with circom circuit (Deferred)

- [ ] Test: `test_encap_with_circom`
- [ ] Load circuit with CircomBuilder
- [ ] Setup circuit without witness
- [ ] Call zkenc-core encap
- [ ] Verify key generation

### Test 3.2: Decap with circom circuit (Deferred)

- [ ] Test: `test_decap_with_circom`
- [ ] Load circuit with full witness
- [ ] Call zkenc-core decap
- [ ] Verify key recovery

### Implementation 3.1: Circuit wrapper (Design Complete)

- [x] Create CircomCircuitWrapper struct (placeholder)
- [x] Document ConstraintSynthesizer implementation plan
- [x] Document actual blocker with compilation error
- [ ] Implementation requires resolving ark-circom dependency conflict

---

## Phase 4: AES Encryption (Test-First)

**Status**: ✅ Complete

**Goal**: Encrypt/decrypt with derived key

### Test 4.1: AES-GCM roundtrip

- [x] Test: `test_aes_gcm_roundtrip`
- [x] Encrypt plaintext
- [x] Decrypt and verify

### Test 4.2: AES-CTR roundtrip

- [x] Test: `test_aes_ctr_roundtrip`

### Implementation 4.1: Crypto module

- [x] Implement `encrypt()` and `decrypt()`
- [x] Support GCM and CTR modes

**Results**: 4 unit tests + 2 integration tests passing

- `crypto::tests::test_gcm_roundtrip` ✅
- `crypto::tests::test_ctr_roundtrip` ✅
- `crypto::tests::test_gcm_wrong_key` ✅
- `crypto::tests::test_invalid_key_length` ✅
- `test_aes_gcm_roundtrip` ✅
- `test_aes_ctr_roundtrip` ✅

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

- Phase 1: ✅ Complete (Circom loading - file validation)
- Phase 2: ✅ Complete (Input parsing - JSON flattening)
- Phase 3: ⚠️ **Blocked** (ark-circom has internal dependency conflicts with git arkworks)
- Phase 4: ✅ Complete (AES encryption - GCM/CTR modes)
- Phase 5: ⏸️ Not started (CLI commands - blocked by Phase 3)

## Current Status

**All independent components complete**. Further progress requires resolving Phase 3 blocker:
- ✅ File I/O (Phase 1)
- ✅ JSON parsing (Phase 2)  
- ✅ AES encryption (Phase 4)
- ⚠️ **Cannot proceed with Phase 5 CLI** without zkenc-core integration

**Next Steps**: Choose a resolution path for Phase 3 before continuing
