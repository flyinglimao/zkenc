# zkenc-core 測試規劃與實作摘要

## 📝 已完成工作

### 1. 技術分析與設計文件 ✅

**輸出**: `packages/zkenc-core/DESIGN.md`

**關鍵發現**:

- WKEM 與 Groth16 共享 R1CS→QAP 轉換邏輯
- 主要差異：CRS 每次 Encap 重新生成 vs Groth16 固定 setup
- 隨機數 `r` 的角色完全不同（加密隨機性 vs zero-knowledge blinding）
- φᵢ(x) = r·β·uᵢ(x) + r·α·vᵢ(x) + r²·wᵢ(x) 是 WKEM 特有結構

### 2. MiMC 測試電路 ✅

**檔案**: `packages/zkenc-core/tests/mimc_circuit.rs`

**實作內容**:

- `MiMCCircuit<F>` struct（包含 xl, xr witness 和 output public input）
- `mimc_native()` 函數（native 計算，用於生成測試向量）
- `ConstraintSynthesizer` 實作（322 rounds，每輪 xL, xR := xR + (xL + Cᵢ)³, xL）
- 單元測試：
  - `test_mimc_native`: 驗證 native 計算
  - `test_mimc_circuit_satisfies`: 正確輸入應滿足約束
  - `test_mimc_circuit_fails_with_wrong_output`: 錯誤輸出應不滿足約束

**編譯狀態**: ⚠️ 需要 `ark-r1cs-std` dev-dependency（已在 with_curves feature 中配置）

### 3. Encap/Decap 整合測試 ✅

**檔案**: `packages/zkenc-core/tests/encap_decap.rs`

**測試案例** (目前標記為 `#[ignore]`，等待實作):

1. **`test_encap_decap_correctness`**: 正確 witness → 相同 key
2. **`test_encap_decap_wrong_witness`**: 錯誤 witness → 不同 key 或錯誤
3. **`test_encap_different_public_inputs`**: 不同 public inputs → 不同 ciphertext
4. **`test_ciphertext_serialization`**: Ciphertext 序列化/反序列化
5. **`test_mimc_circuit_integration`**: MiMC 電路獨立驗證（不需 encap/decap，應立即通過）

## 🎯 下一步實作計畫

### Phase 4: 數據結構實作

**檔案**: `packages/zkenc-core/src/data_structures.rs`

**需要定義**:

```rust
pub struct EncapKey<E: Pairing> {
    pub alpha_g1: E::G1Affine,
    pub beta_g2: E::G2Affine,
    pub delta_g2: E::G2Affine,
    pub r_u_query_g1: Vec<E::G1Affine>,        // {[r·uᵢ(x)]₁}
    pub r_v_query_g2: Vec<E::G2Affine>,        // {[r·vᵢ(x)]₂}
    pub phi_delta_query_g1: Vec<E::G1Affine>,  // {[φᵢ(x)/δ]₁} for witness
    pub h_query_g1: Vec<E::G1Affine>,          // {[r²·xⁱ·t(x)/δ]₁}
}

pub struct Ciphertext<E: Pairing> {
    pub encap_key: EncapKey<E>,
    pub public_inputs: Vec<E::ScalarField>,
}

pub struct Key(pub [u8; 32]);  // Keccak256 output

impl<E: Pairing> CanonicalSerialize for EncapKey<E> { ... }
impl<E: Pairing> CanonicalDeserialize for EncapKey<E> { ... }
```

### Phase 5: Encap 演算法骨架

**檔案**: `packages/zkenc-core/src/algorithm.rs`

**步驟**:

1. 採樣隨機數 α, β, δ, r, x
2. Synthesize circuit → R1CS
3. R1CS → QAP（使用 `LibsnarkReduction::instance_map_with_evaluation`）
4. 計算 CRS 各組件（參考 Groth16 generator.rs 的 MSM 模式）
5. 計算 s = [α]₁·[β]₂ + Σ aᵢ[φᵢ(x)]₁·[1]₂
6. k ← Keccak256(serialize(s))

**重用 Groth16 程式碼**:

- Domain 建立與 FFT
- MSM (batch_mul)
- QAP 轉換

### Phase 6: Decap 演算法骨架

**步驟**:

1. 從 Ciphertext 解析 EncapKey
2. Synthesize circuit with witness → R1CS
3. R1CS → QAP witness map（得到 h(x)）
4. 計算 A = [α]₁ + Σ aᵢ[r·uᵢ(x)]₁
5. 計算 B = [β]₂ + Σ aᵢ[r·vᵢ(x)]₂
6. 計算 C = Σ aᵢ[φᵢ(x)/δ]₁ + [r²·h(x)·t(x)/δ]₁
7. 計算 s = pairing(A, B) - pairing(C, [δ]₂)
8. k ← Keccak256(serialize(s))

### Phase 7: 測試驅動迭代

**流程**:

1. 移除測試中的 `#[ignore]` 標記
2. `cargo test -p zkenc-core --features with_curves`
3. 根據失敗訊息修正實作
4. 重複 2-3 直到所有測試通過

## 📊 預期測試結果

| 測試                                 | 預期狀態      | 驗證內容              |
| ------------------------------------ | ------------- | --------------------- |
| `test_mimc_circuit_integration`      | ✅ 立即通過   | MiMC 電路正確性       |
| `test_encap_decap_correctness`       | ⏳ 實作後通過 | 正確 witness 復原 key |
| `test_encap_decap_wrong_witness`     | ⏳ 實作後通過 | 錯誤 witness 偵測     |
| `test_encap_different_public_inputs` | ⏳ 實作後通過 | Ciphertext 唯一性     |
| `test_ciphertext_serialization`      | ⏳ 實作後通過 | 序列化正確性          |

## 🔧 依賴項配置

**已添加到 `Cargo.toml`**:

```toml
[features]
with_curves = ["ark-bls12-381", "std", "r1cs"]

[dependencies]
# (現有依賴保持不變)

# 需要添加 Keccak256:
tiny-keccak = { version = "2.0", features = ["keccak"], optional = true }

[dev-dependencies]
ark-bls12-381 = { git = "https://github.com/arkworks-rs/algebra.git", optional = true }
```

**Feature 啟用時包含**:

- BLS12-381 curve
- R1CS gadgets (ark-r1cs-std)
- Standard library

## 📚 參考實作對照

| 功能                | Groth16 檔案     | WKEM 對應                        |
| ------------------- | ---------------- | -------------------------------- |
| Setup               | `generator.rs`   | `encap()` 的 CRS 生成部分        |
| Prove               | `prover.rs`      | `decap()` 的 witness computation |
| R1CS→QAP            | `r1cs_to_qap.rs` | 兩者共用（完全相同）             |
| Pairing computation | `verifier.rs`    | `encap()`/`decap()` 的 pairing   |

## ⚠️ 實作注意事項

1. **φᵢ(x) 計算**: 注意 r 的次方（r¹·β·u + r¹·α·v + r²·w）
2. **Domain size**: 必須 ≥ num_constraints + num_instance_variables
3. **Public inputs indexing**: a₀ = 1 (固定), a₁..aℓ 是實際 public inputs
4. **h(x) 係數**: witness_map 返回的是 evaluation form，需要正確索引
5. **Pairing 順序**: e(A,B) - e(C,δ) = e(A,B) · e(C,δ)⁻¹（群運算）

## 🚀 執行指令

```bash
# 測試 MiMC 電路（不需 encap/decap）
cargo test -p zkenc-core --features with_curves test_mimc

# 執行所有測試（encap/decap 實作後）
cargo test -p zkenc-core --features with_curves

# 執行輕量測試（預設，不拉 curve crate）
cargo test -p zkenc-core
```

---

**版本**: v0.1.0  
**建立日期**: 2025-10-11  
**狀態**: Phase 1-3 完成，Phase 4-7 待實作
