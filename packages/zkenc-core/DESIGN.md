# zkenc-core 技術設計文件

## 專案概述

本專案實作基於 QAP (Quadratic Arithmetic Programs) 的可提取見證金鑰封裝機制 (Extractable Witness Key Encapsulation Mechanism, WKEM)。

**目標**: 提供 `encap` 和 `decap` 兩個核心 API，使用 BLS12-381 曲線和 Keccak256 雜湊函數。

## 與 Groth16 的關係

### 相似之處

- **R1CS → QAP 轉換**: 完全相同的流程
- **多項式運算**: FFT/IFFT、domain evaluation、witness map
- **橢圓曲線運算**: MSM、pairing 計算
- **約束系統**: 使用 `ark-relations` 的 `ConstraintSynthesizer` trait

### 關鍵差異

| 面向         | Groth16                    | WKEM (本專案)                |
| ------------ | -------------------------- | ---------------------------- |
| **用途**     | 零知識證明                 | 見證加密                     |
| **API**      | setup/prove/verify         | encap/decap                  |
| **CRS**      | 固定參數，一次 setup       | 每次 encap 重新採樣          |
| **隨機數 r** | Zero-knowledge blinding    | 加密隨機性（保護多項式評估） |
| **輸出**     | Proof                      | (Ciphertext, Key)            |
| **安全性**   | Zero-knowledge + Soundness | Correctness + Extractability |

## 數學結構

### Encap 演算法

**輸入**:

- QAP `Q` (隱式，透過 circuit)
- Public inputs `{aᵢ}ᵢ₌₀^ℓ`

**輸出**:

- Ciphertext `ct = (σ, {aᵢ}ᵢ₌₀^ℓ)`
- Key `k ∈ 𝒦`

**步驟**:

1. 採樣隨機數: `α, β, δ, r, x ← 𝔽p*`
2. 執行 R1CS → QAP 轉換，得到多項式 {uᵢ(X), vᵢ(X), wᵢ(X)}
3. 計算 CRS σ:
   ```
   σ = {
       [α]₁, [β]₂, [δ]₂,
       {[r·uᵢ(x)]₁}ᵢ₌₀^m,
       {[r·vᵢ(x)]₂}ᵢ₌₀^m,
       {[φᵢ(x)/δ]₁}ᵢ₌ℓ₊₁^m,
       {[r²·xⁱ·t(x)/δ]₁}ᵢ₌₀^(n-2)
   }
   ```
   其中 `φᵢ(x) = r·β·uᵢ(x) + r·α·vᵢ(x) + r²·wᵢ(x)`
4. 計算封裝值:
   ```
   s = [α]₁ · [β]₂ + Σᵢ₌₀^ℓ aᵢ·[φᵢ(x)]₁ · [1]₂
   ```
5. 派生金鑰: `k ← H(s)` (H: 𝔾T → 𝒦)

### Decap 演算法

**輸入**:

- QAP `Q`
- Witness `{aᵢ}ᵢ₌ℓ₊₁^m`
- Ciphertext `ct = (σ, {aᵢ}ᵢ₌₀^ℓ)`

**輸出**: Key `k`

**步驟**:

1. 從 σ 解析出各個群元素
2. 計算 A, B, C:
   ```
   A = [α]₁ + Σᵢ₌₀^m aᵢ·[r·uᵢ(x)]₁
   B = [β]₂ + Σᵢ₌₀^m aᵢ·[r·vᵢ(x)]₂
   C = Σᵢ₌ℓ₊₁^m aᵢ·[φᵢ(x)/δ]₁ + [r²·h(x)·t(x)/δ]₁
   ```
   其中 h(X) 是商多項式，滿足:
   ```
   Σ aᵢ·uᵢ(X) · Σ aᵢ·vᵢ(X) - Σ aᵢ·wᵢ(X) = h(X)·t(X)
   ```
3. 計算封裝值:
   ```
   s = A · B - C · [δ]₂
   ```
4. 派生金鑰: `k ← H(s)`

### 正確性

當 witness 有效（即 QAP 關係滿足）時，Encap 和 Decap 應計算出相同的 `s`，因此得到相同的 `k`。

**關鍵等式**:

```
A·B - C·[δ]₂
= ([α]₁ + Σaᵢ[ruᵢ(x)]₁)·([β]₂ + Σaᵢ[rvᵢ(x)]₂) - (...)·[δ]₂
= [α]₁·[β]₂ + Σaᵢ[φᵢ(x)]₁·[1]₂  (當 QAP 滿足時)
= s (from Encap)
```

## 實作計畫

### Phase 1: 數據結構 (packages/zkenc-core/src/data_structures.rs)

```rust
pub struct EncapKey<E: Pairing> {
    // CRS σ
    pub alpha_g1: E::G1Affine,
    pub beta_g2: E::G2Affine,
    pub delta_g2: E::G2Affine,

    // {[r·uᵢ(x)]₁}
    pub r_u_query_g1: Vec<E::G1Affine>,

    // {[r·vᵢ(x)]₂}
    pub r_v_query_g2: Vec<E::G2Affine>,

    // {[φᵢ(x)/δ]₁} for i > ℓ (witness part)
    pub phi_delta_query_g1: Vec<E::G1Affine>,

    // {[r²·xⁱ·t(x)/δ]₁} for h(x) computation
    pub h_query_g1: Vec<E::G1Affine>,
}

pub struct Ciphertext<E: Pairing> {
    pub encap_key: EncapKey<E>,
    pub public_inputs: Vec<E::ScalarField>,
}

pub struct Key([u8; 32]);  // Keccak256 輸出
```

### Phase 2: 演算法實作 (packages/zkenc-core/src/algorithm.rs)

```rust
pub fn encap<E, C, R>(
    circuit: C,
    public_inputs: &[E::ScalarField],
    rng: &mut R,
) -> Result<(Ciphertext<E>, Key), Error>
where
    E: Pairing,
    C: ConstraintSynthesizer<E::ScalarField>,
    R: RngCore,
{
    // 1. Sample randomness
    // 2. Synthesize circuit & get R1CS
    // 3. R1CS → QAP (reuse arkworks)
    // 4. Compute σ (CRS components)
    // 5. Compute s via pairing
    // 6. k ← Keccak256(s)
}

pub fn decap<E, C>(
    circuit: C,
    witness: &[E::ScalarField],
    ciphertext: &Ciphertext<E>,
) -> Result<Key, Error>
where
    E: Pairing,
    C: ConstraintSynthesizer<E::ScalarField>,
{
    // 1. Parse σ from ciphertext
    // 2. Compute h(x) via witness_map
    // 3. Compute A, B, C
    // 4. Compute s = A·B - C·δ
    // 5. k ← Keccak256(s)
}
```

### Phase 3: 測試電路 (packages/zkenc-core/tests/mimc_circuit.rs)

使用 MiMC (LongsightF322p3) 作為測試電路：

- 322 rounds
- xL, xR := xR + (xL + Cᵢ)³, xL
- Public input: output
- Witness: xL, xR (preimage)

### Phase 4: 整合測試 (packages/zkenc-core/tests/encap_decap.rs)

測試案例：

1. **正確性測試**: 正確 witness → 相同 key
2. **錯誤 witness 測試**: 錯誤 witness → 不同 key 或錯誤
3. **公開輸入不匹配**: 不同 public inputs → 不同 ciphertext
4. **序列化/反序列化**: Ciphertext 可以正確序列化

## 依賴項對應

| 功能       | Groth16 使用                             | WKEM 使用                     |
| ---------- | ---------------------------------------- | ----------------------------- |
| R1CS → QAP | `r1cs_to_qap::LibsnarkReduction`         | 同左（可直接重用）            |
| Domain     | `GeneralEvaluationDomain`                | 同左                          |
| MSM        | `VariableBaseMSM::msm_bigint`            | 同左                          |
| Pairing    | `E::pairing()`, `E::multi_miller_loop()` | 同左                          |
| Hash       | N/A                                      | `tiny-keccak` (SHA3-256 mode) |

## 開發流程 (TDD)

1. ✅ 建立此設計文件
2. ⬜ 實作 MiMC 測試電路
3. ⬜ 撰寫失敗的 encap/decap 測試
4. ⬜ 實作數據結構
5. ⬜ 實作 encap 骨架（先返回 dummy 值）
6. ⬜ 實作 decap 骨架
7. ⬜ 逐步完善實作直到測試通過
8. ⬜ 添加 edge case 測試

## 參考資料

- **論文**: Scheme.tex (本專案根目錄)
- **Groth16 實作**: https://github.com/arkworks-rs/groth16
- **MiMC 測試**: groth16/tests/mimc.rs
- **R1CS to QAP**: groth16/src/r1cs_to_qap.rs

## 注意事項

1. **安全隨機數**: α, β, δ, r, x 必須使用密碼學安全的 RNG
2. **Domain size**: 必須 ≥ num_constraints + num_instance_variables
3. **φᵢ(x) 計算**: 注意 r 的不同次方（r·β·u + r·α·v + r²·w）
4. **h(x) 計算**: witness_map 返回的係數即為 h(X) 的 evaluation form
5. **Pairing 順序**: GT 的加法對應 G1×G2 的 pairing 乘法

---

**版本**: v0.1.0  
**最後更新**: 2025-10-11
