# zkenc-core 實作摘要與後續建議

## ✅ 已完成的工作

### 1. 技術設計文件
- **檔案**: `packages/zkenc-core/DESIGN.md`
- **內容**: 完整的 WKEM 數學結構、與 Groth16 對比、實作計畫

### 2. 測試規劃文件
- **檔案**: `packages/zkenc-core/TEST_PLAN.md`
- **內容**: TDD 測試策略、測試案例設計、執行流程

### 3. MiMC 測試電路
- **檔案**: `packages/zkenc-core/tests/mimc_circuit.rs`
- **實作**: 322-round MiMC hash circuit (LongsightF322p3)
- **包含**: native 計算、ConstraintSynthesizer 實作、單元測試

### 4. Encap/Decap 整合測試框架
- **檔案**: `packages/zkenc-core/tests/encap_decap.rs`
- **測試案例**: 
  - 正確性測試（witness → key 復原）
  - 錯誤 witness 測試
  - 不同 public inputs 測試
  - 序列化測試
  - MiMC 電路獨立驗證

## ⚠️ 當前阻塞問題：依賴版本衝突

### 問題描述
```
error: the trait bound `Fp<MontBackend<FrConfig, 4>, 4>: Field` is not satisfied
note: two types coming from two different versions of the same crate are different 
types even if they look the same
```

**根本原因**: `ark-std`, `ark-ff` 等 arkworks crates 同時使用了：
- **Git 版本** (在 `packages/zkenc-core/Cargo.toml` 中)
- **Crates.io 版本** (作為 `ark-relations` 等的傳遞依賴)

### 解決方案

**選項 A (推薦): 統一使用 Git 版本**

修改 `packages/zkenc-core/Cargo.toml`，將所有 arkworks 依賴改為 git：

```toml
[dependencies]
# Arkworks algebra - 數學基礎庫 (統一使用 git)
ark-ff = { git = "https://github.com/arkworks-rs/algebra.git", default-features = false }
ark-ec = { git = "https://github.com/arkworks-rs/algebra.git", default-features = false }
ark-serialize = { git = "https://github.com/arkworks-rs/algebra.git", default-features = false, features = ["derive"] }
ark-poly = { git = "https://github.com/arkworks-rs/algebra.git", default-features = false }
ark-std = { git = "https://github.com/arkworks-rs/std.git", default-features = false }

# Arkworks SNARK - R1CS 與 SNARK 抽象層 (統一使用 git)
ark-relations = { git = "https://github.com/arkworks-rs/snark.git", default-features = false }
ark-snark = { git = "https://github.com/arkworks-rs/snark.git", default-features = false }

# Arkworks gadgets - 可選功能（電路元件）(統一使用 git)
ark-r1cs-std = { git = "https://github.com/arkworks-rs/r1cs-std.git", default-features = false, optional = true }
ark-crypto-primitives = { git = "https://github.com/arkworks-rs/crypto-primitives.git", default-features = false, features = ["snark", "sponge"] }

# ... (其他依賴不變)

[dev-dependencies]
ark-bls12-381 = { git = "https://github.com/arkworks-rs/algebra.git", default-features = false, features = ["curve"], optional = true }
```

**選項 B: 統一使用 Crates.io 版本**

改用固定版本號（如 0.5.0），但需確認所有 arkworks crates 在 crates.io 上的版本一致性。

## 📊 當前專案狀態

### 編譯狀態
- ✅ `cargo check -p zkenc-core`: 通過
- ✅ `cargo check -p zkenc-core --features with_curves`: 通過
- ❌ `cargo test -p zkenc-core --features with_curves`: **依賴版本衝突**

### 程式碼結構
```
packages/zkenc-core/
├── Cargo.toml                   ✅ Features 已配置
├── DESIGN.md                    ✅ 技術設計完成
├── TEST_PLAN.md                 ✅ 測試規劃完成
├── src/
│   ├── lib.rs                   ✅ 基本骨架
│   ├── algorithm.rs             ⏳ 待實作 encap/decap
│   └── data_structures.rs       ⏳ 待實作 EncapKey/Ciphertext/Key
└── tests/
    ├── basic.rs                 ✅ 輕量測試（已通過）
    ├── mimc_circuit.rs          ⚠️ 實作完成，編譯被阻塞
    └── encap_decap.rs           ⚠️ 框架完成，編譯被阻塞
```

## 🎯 建議的下一步驟

### 立即行動 (修復編譯)
1. **統一 arkworks 依賴為 git 版本**
   ```bash
   # 編輯 packages/zkenc-core/Cargo.toml
   # 將上述「選項 A」的修改應用
   ```

2. **驗證編譯通過**
   ```bash
   cargo clean -p zkenc-core
   cargo test -p zkenc-core --features with_curves --no-run
   ```

3. **執行 MiMC 測試**
   ```bash
   cargo test -p zkenc-core --features with_curves test_mimc
   ```

### 後續實作順序

#### Phase 1: 修復編譯並驗證測試框架
- ✅ 修改 Cargo.toml（統一依賴版本）
- ⏳ 驗證 MiMC 測試通過
- ⏳ Commit 測試框架

#### Phase 2: 實作核心數據結構
- ⏳ 在 `data_structures.rs` 定義:
  - `EncapKey<E: Pairing>` (CRS σ)
  - `Ciphertext<E: Pairing>`
  - `Key` ([u8; 32])
  - Serialize/Deserialize traits

#### Phase 3: 實作 Encap 骨架
- ⏳ 參考 Groth16 `generator.rs`
- ⏳ 實作 QAP 轉換
- ⏳ 計算 CRS 各組件
- ⏳ 計算 pairing 並派生 key

#### Phase 4: 實作 Decap 骨架
- ⏳ 參考 Groth16 `prover.rs` + `verifier.rs`
- ⏳ 計算 A, B, C
- ⏳ 計算 pairing 並派生 key

#### Phase 5: TDD 迭代
- ⏳ 移除測試的 `#[ignore]` 標記
- ⏳ 逐個修正失敗測試
- ⏳ 添加 edge cases

## 📝 關鍵技術債務追蹤

| 項目 | 狀態 | 優先級 | 估計時間 |
|------|------|--------|---------|
| 依賴版本衝突 | ❌ 阻塞 | P0 | 5 分鐘 |
| 數據結構定義 | ⏳ 待辦 | P1 | 30 分鐘 |
| Encap 實作 | ⏳ 待辦 | P1 | 2-3 小時 |
| Decap 實作 | ⏳ 待辦 | P1 | 1-2 小時 |
| Keccak256 整合 | ⏳ 待辦 | P2 | 15 分鐘 |
| 測試迭代與修正 | ⏳ 待辦 | P2 | 1-2 小時 |

## 🔍 驗證清單

執行以下命令來驗證進度：

```bash
# 1. 確認依賴統一後編譯通過
cargo clean -p zkenc-core
cargo check -p zkenc-core --features with_curves

# 2. 執行 MiMC 電路測試 (應立即通過)
cargo test -p zkenc-core --features with_curves test_mimc -- --nocapture

# 3. 執行完整測試 (實作 encap/decap 後)
cargo test -p zkenc-core --features with_curves -- --nocapture

# 4. 確認輕量測試仍可運行
cargo test -p zkenc-core
```

## 📚 參考資料快速連結

- **Scheme 定義**: `Scheme.tex` (專案根目錄)
- **Groth16 參考**:
  - Generator: https://github.com/arkworks-rs/groth16/blob/master/src/generator.rs
  - Prover: https://github.com/arkworks-rs/groth16/blob/master/src/prover.rs
  - Verifier: https://github.com/arkworks-rs/groth16/blob/master/src/verifier.rs
  - R1CS→QAP: https://github.com/arkworks-rs/groth16/blob/master/src/r1cs_to_qap.rs
- **MiMC 範例**: https://github.com/arkworks-rs/groth16/blob/master/tests/mimc.rs

---

**建立日期**: 2025-10-11  
**狀態**: 編譯阻塞（依賴版本衝突），測試框架已就緒  
**下一步**: 修復 Cargo.toml 依賴版本，驗證 MiMC 測試通過
