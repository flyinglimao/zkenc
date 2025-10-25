---
sidebar_position: 1
---

# zkenc-core API 參考

zkenc-core 是提供見證加密密碼學原語的 Rust 函式庫。它作為基礎層 - zkenc-cli 和 zkenc-js 都建構在這個核心函式庫之上。

## 概覽

zkenc-core 使用橢圓曲線密碼學（BN254 曲線）實作 R1CS 電路的見證加密。它提供兩個核心函式：

- **`encap`**：產生見證加密的金鑰
- **`decap`**：使用有效見證恢復金鑰

## 安裝

新增至您的 `Cargo.toml`：

```toml
[dependencies]
zkenc-core = { path = "../zkenc-core" }
ark-bn254 = "0.4"
ark-std = "0.4"
ark-serialize = "0.4"
```

## 核心型別

### `Ciphertext<E: Pairing>`

代表見證加密密文。

```rust
pub struct Ciphertext<E: Pairing> {
    // 內部欄位為私有
}
```

**屬性：**

- 可使用 arkworks 序列化進行序列化
- 大小：BN254 曲線約 1576 位元組
- 可透過網路傳送或儲存至磁碟

### `Key`

代表對稱加密金鑰。

```rust
pub struct Key {
    // 內部：32 位元組金鑰
}
```

**方法：**

- `as_bytes() -> &[u8; 32]` - 取得金鑰的位元組表示以進行 AES 加密

## 核心函式

### `encap`

從電路產生見證加密的密文和金鑰。

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

**型別參數：**

- `E` - 橢圓曲線配對（通常為 `Bn254`）
- `C` - 實作 `ConstraintSynthesizer` 的電路
- `R` - 隨機數產生器

**參數：**

- `circuit` - 已設定公開輸入的電路實例
- `rng` - 密碼學安全的隨機數產生器

**回傳：**

- `Ok((Ciphertext, Key))` - 密文和產生的金鑰
- `Err(EncapError)` - 如果電路合成失敗

**範例：**

```rust
use zkenc_core::{encap, Ciphertext, Key};
use ark_bn254::Bn254;
use ark_std::rand::rngs::OsRng;

// 建立包含公開輸入的電路
let mut circuit = MyCircuit::new();
circuit.set_public_input(42);

// 產生密文和金鑰
let mut rng = OsRng;
let (ciphertext, key) = encap::<Bn254, _, _>(circuit, &mut rng)?;

// 序列化以供儲存/傳輸
let mut ct_bytes = Vec::new();
ciphertext.serialize_compressed(&mut ct_bytes)?;
```

**效能：**

- 時間複雜度：O(n * log n)，其中 n = 約束數量
- 記憶體：O(n) 用於約束系統
- 典型時間：50-500ms，取決於電路大小

### `decap`

使用有效見證恢復加密金鑰。

```rust
pub fn decap<E, C>(
    circuit: C,
    ciphertext: &Ciphertext<E>,
) -> Result<Key, DecapError>
where
    E: Pairing,
    C: ConstraintSynthesizer<E::ScalarField>,
```

**型別參數：**

- `E` - 橢圓曲線配對（通常為 `Bn254`）
- `C` - 實作 `ConstraintSynthesizer` 的電路

**參數：**

- `circuit` - 已設定完整見證的電路實例
- `ciphertext` - 來自 `encap` 的密文

**回傳：**

- `Ok(Key)` - 恢復的加密金鑰
- `Err(DecapError)` - 如果見證無效或不滿足約束

**範例：**

```rust
use zkenc_core::{decap, Ciphertext, Key};
use ark_bn254::Bn254;
use ark_serialize::CanonicalDeserialize;

// 載入密文
let ct_bytes = std::fs::read("ciphertext.bin")?;
let ciphertext = Ciphertext::<Bn254>::deserialize_compressed(&ct_bytes[..])?;

// 建立包含完整見證的電路
let mut circuit = MyCircuit::new();
circuit.set_public_input(42);
circuit.set_private_input(123); // 見證

// 恢復金鑰
let key = decap::<Bn254, _>(circuit, &ciphertext)?;

// 使用金鑰進行解密
let key_bytes = key.as_bytes();
```

**效能：**

- 時間複雜度：O(n)，其中 n = 約束數量
- 記憶體：O(n) 用於約束系統
- 典型時間：100-1000ms，取決於電路大小

## 錯誤型別

### `EncapError`

封裝期間可能發生的錯誤。

```rust
pub enum EncapError {
    /// 電路合成失敗
    SynthesisError(SynthesisError),
    /// 隨機數產生失敗
    RngError,
}
```

### `DecapError`

解封裝期間可能發生的錯誤。

```rust
pub enum DecapError {
    /// 電路合成失敗
    SynthesisError(SynthesisError),
    /// 見證不滿足約束
    InvalidWitness,
    /// 配對檢查失敗
    PairingCheckFailed,
}
```

## 電路介面

要使用 zkenc-core，您的電路必須實作 `ConstraintSynthesizer`：

```rust
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};

pub struct MyCircuit<F: Field> {
    pub public_input: Option<F>,
    pub private_input: Option<F>,
}

impl<F: Field> ConstraintSynthesizer<F> for MyCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        // 配置公開輸入
        let pub_var = cs.new_input_variable(|| {
            self.public_input.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // 配置私密輸入（見證）
        let priv_var = cs.new_witness_variable(|| {
            self.private_input.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // 新增約束
        // ... 您的電路邏輯

        Ok(())
    }
}
```

## 完整範例

這是一個完整的加密和解密範例：

```rust
use zkenc_core::{encap, decap};
use ark_bn254::Bn254;
use ark_ff::Field;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_r1cs_std::prelude::*;
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};
use ark_std::rand::rngs::OsRng;

// 定義電路
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

        // 公開輸入
        let pub_var = cs.new_input_variable(|| {
            self.pub_input.map(Fr::from).ok_or(SynthesisError::AssignmentMissing)
        })?;

        // 私密見證
        let priv_var = cs.new_witness_variable(|| {
            self.priv_input.map(Fr::from).ok_or(SynthesisError::AssignmentMissing)
        })?;

        // 約束：pub + priv = 常數
        cs.enforce_constraint(
            lc!() + pub_var + priv_var,
            lc!() + Variable::One,
            lc!() + (165u64, Variable::One), // pub + priv = 165
        )?;

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加密
    println!("加密中...");

    // 建立僅包含公開輸入的電路
    let encrypt_circuit = SimpleCircuit {
        pub_input: Some(42),
        priv_input: None, // 加密不需要
    };

    // 封裝
    let mut rng = OsRng;
    let (ciphertext, key) = encap::<Bn254, _, _>(encrypt_circuit, &mut rng)?;

    println!("產生的金鑰：{:?}", key.as_bytes());

    // 序列化密文
    let mut ct_bytes = Vec::new();
    ciphertext.serialize_compressed(&mut ct_bytes)?;
    std::fs::write("ciphertext.bin", &ct_bytes)?;

    // 使用金鑰進行 AES 加密
    let message = b"秘密訊息";
    // ... 使用 key.as_bytes() 加密訊息 ...

    // 解密
    println!("\n解密中...");

    // 載入密文
    let ct_bytes = std::fs::read("ciphertext.bin")?;
    let ciphertext = Ciphertext::<Bn254>::deserialize_compressed(&ct_bytes[..])?;

    // 建立包含完整見證的電路
    let decrypt_circuit = SimpleCircuit {
        pub_input: Some(42),
        priv_input: Some(123), // 必須滿足：42 + 123 = 165
    };

    // 解封裝
    let recovered_key = decap::<Bn254, _>(decrypt_circuit, &ciphertext)?;

    println!("恢復的金鑰：{:?}", recovered_key.as_bytes());

    // 驗證金鑰是否相符
    assert_eq!(key.as_bytes(), recovered_key.as_bytes());
    println!("✅ 金鑰相符！");

    Ok(())
}
```

## 與其他工具整合

### 與 zkenc-cli 整合

zkenc-cli 內部使用 zkenc-core：

```rust
use zkenc_core::{encap, decap};
use zkenc_cli::circuit::CircomCircuit;

// 載入 R1CS 電路
let circuit = CircomCircuit::from_r1cs("circuit.r1cs")?;

// 封裝
let (ciphertext, key) = encap::<Bn254, _, _>(circuit, &mut rng)?;
```

### 與自訂電路整合

```rust
// 您的自訂電路
impl ConstraintSynthesizer<Fr> for MyCircuit {
    fn generate_constraints(/* ... */) -> Result<(), SynthesisError> {
        // 您的約束
    }
}

// 與 zkenc-core 一起使用
let circuit = MyCircuit::new(/* 參數 */);
let (ct, key) = encap::<Bn254, _, _>(circuit, &mut rng)?;
```

## 效能提示

1. **電路最佳化**：最小化約束以加快操作速度
2. **批次操作**：盡可能重複使用電路編譯
3. **記憶體管理**：使用壓縮序列化以減少大小
4. **平行處理**：encap/decap 可在多個訊息之間平行化

## 曲線選擇

目前，zkenc-core 預設使用 BN254 曲線：

```rust
use ark_bn254::Bn254;

let (ct, key) = encap::<Bn254, _, _>(circuit, &mut rng)?;
```

未來版本可能支援額外的曲線（BLS12-381 等）。

## 安全性考量

1. **RNG 安全性**：始終使用密碼學安全的 RNG（`OsRng`）
2. **見證隱私**：永遠不要暴露私密見證值
3. **電路正確性**：確保電路正確執行約束
4. **金鑰使用**：使用適當的對稱加密（AES-256-GCM）來使用金鑰

## 下一步

- **[zkenc-cli API →](/docs/api/zkenc-cli)** - 命令列介面
- **[zkenc-js API →](/docs/api/zkenc-js)** - JavaScript 綁定
- **[快速入門 →](/docs/getting-started/zkenc-cli)** - 快速入門指南
