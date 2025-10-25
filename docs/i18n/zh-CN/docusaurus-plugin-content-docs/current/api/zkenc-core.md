---
sidebar_position: 1
---

# zkenc-core API 参考

zkenc-core 是提供见证加密密码学原语的 Rust 库。它作为基础层 - zkenc-cli 和 zkenc-js 都建构在这个核心库之上。

## 概览

zkenc-core 使用椭圆曲线密码学（BN254 曲线）实现 R1CS 电路的见证加密。它提供两个核心函数：

- **`encap`**：生成见证加密的密钥
- **`decap`**：使用有效见证恢复密钥

## 安装

添加至您的 `Cargo.toml`：

```toml
[dependencies]
zkenc-core = { path = "../zkenc-core" }
ark-bn254 = "0.4"
ark-std = "0.4"
ark-serialize = "0.4"
```

## 核心类型

### `Ciphertext<E: Pairing>`

代表见证加密密文。

```rust
pub struct Ciphertext<E: Pairing> {
    // 内部字段为私有
}
```

**属性：**

- 可使用 arkworks 序列化进行序列化
- 大小：BN254 曲线约 1576 字节
- 可透过网络传送或存储至磁盘

### `Key`

代表对称加密密钥。

```rust
pub struct Key {
    // 内部：32 字节密钥
}
```

**方法：**

- `as_bytes() -> &[u8; 32]` - 获取密钥的字节表示以进行 AES 加密

## 核心函数

### `encap`

从电路生成见证加密的密文和密钥。

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

**类型参数：**

- `E` - 椭圆曲线配对（通常为 `Bn254`）
- `C` - 实现 `ConstraintSynthesizer` 的电路
- `R` - 随机数生成器

**参数：**

- `circuit` - 已设置公开输入的电路实例
- `rng` - 密码学安全的随机数生成器

**返回：**

- `Ok((Ciphertext, Key))` - 密文和生成的密钥
- `Err(EncapError)` - 如果电路合成失败

**范例：**

```rust
use zkenc_core::{encap, Ciphertext, Key};
use ark_bn254::Bn254;
use ark_std::rand::rngs::OsRng;

// 建立包含公开输入的电路
let mut circuit = MyCircuit::new();
circuit.set_public_input(42);

// 生成密文和密钥
let mut rng = OsRng;
let (ciphertext, key) = encap::<Bn254, _, _>(circuit, &mut rng)?;

// 序列化以供存储/传输
let mut ct_bytes = Vec::new();
ciphertext.serialize_compressed(&mut ct_bytes)?;
```

**效能：**

- 时间复杂度：O(n \* log n)，其中 n = 约束数量
- 内存：O(n) 用于约束系统
- 典型时间：50-500ms，取决于电路大小

### `decap`

使用有效见证恢复加密密钥。

```rust
pub fn decap<E, C>(
    circuit: C,
    ciphertext: &Ciphertext<E>,
) -> Result<Key, DecapError>
where
    E: Pairing,
    C: ConstraintSynthesizer<E::ScalarField>,
```

**类型参数：**

- `E` - 椭圆曲线配对（通常为 `Bn254`）
- `C` - 实现 `ConstraintSynthesizer` 的电路

**参数：**

- `circuit` - 已设置完整见证的电路实例
- `ciphertext` - 来自 `encap` 的密文

**返回：**

- `Ok(Key)` - 恢复的加密密钥
- `Err(DecapError)` - 如果见证无效或不满足约束

**范例：**

```rust
use zkenc_core::{decap, Ciphertext, Key};
use ark_bn254::Bn254;
use ark_serialize::CanonicalDeserialize;

// 载入密文
let ct_bytes = std::fs::read("ciphertext.bin")?;
let ciphertext = Ciphertext::<Bn254>::deserialize_compressed(&ct_bytes[..])?;

// 建立包含完整见证的电路
let mut circuit = MyCircuit::new();
circuit.set_public_input(42);
circuit.set_private_input(123); // 见证

// 恢复密钥
let key = decap::<Bn254, _>(circuit, &ciphertext)?;

// 使用密钥进行解密
let key_bytes = key.as_bytes();
```

**效能：**

- 时间复杂度：O(n)，其中 n = 约束数量
- 内存：O(n) 用于约束系统
- 典型时间：100-1000ms，取决于电路大小

## 错误类型

### `EncapError`

封装期间可能发生的错误。

```rust
pub enum EncapError {
    /// 电路合成失败
    SynthesisError(SynthesisError),
    /// 随机数生成失败
    RngError,
}
```

### `DecapError`

解封装期间可能发生的错误。

```rust
pub enum DecapError {
    /// 电路合成失败
    SynthesisError(SynthesisError),
    /// 见证不满足约束
    InvalidWitness,
    /// 配对检查失败
    PairingCheckFailed,
}
```

## 电路接口

要使用 zkenc-core，您的电路必须实现 `ConstraintSynthesizer`：

```rust
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};

pub struct MyCircuit<F: Field> {
    pub public_input: Option<F>,
    pub private_input: Option<F>,
}

impl<F: Field> ConstraintSynthesizer<F> for MyCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        // 配置公开输入
        let pub_var = cs.new_input_variable(|| {
            self.public_input.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // 配置私密输入（见证）
        let priv_var = cs.new_witness_variable(|| {
            self.private_input.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // 添加约束
        // ... 您的电路逻辑

        Ok(())
    }
}
```
## 完整范例

这是一个完整的加密和解密范例：

```rust
use zkenc_core::{encap, decap};
use ark_bn254::Bn254;
use ark_ff::Field;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_r1cs_std::prelude::*;
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};
use ark_std::rand::rngs::OsRng;

// 定义电路
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

        // 公开输入
        let pub_var = cs.new_input_variable(|| {
            self.pub_input.map(Fr::from).ok_or(SynthesisError::AssignmentMissing)
        })?;

        // 私密见证
        let priv_var = cs.new_witness_variable(|| {
            self.priv_input.map(Fr::from).ok_or(SynthesisError::AssignmentMissing)
        })?;

        // 约束：pub + priv = 常数
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

    // 建立仅包含公开输入的电路
    let encrypt_circuit = SimpleCircuit {
        pub_input: Some(42),
        priv_input: None, // 加密不需要
    };

    // 封装
    let mut rng = OsRng;
    let (ciphertext, key) = encap::<Bn254, _, _>(encrypt_circuit, &mut rng)?;

    println!("生成的密钥：{:?}", key.as_bytes());

    // 序列化密文
    let mut ct_bytes = Vec::new();
    ciphertext.serialize_compressed(&mut ct_bytes)?;
    std::fs::write("ciphertext.bin", &ct_bytes)?;

    // 使用密钥进行 AES 加密
    let message = b"秘密消息";
    // ... 使用 key.as_bytes() 加密消息 ...

    // 解密
    println!("\n解密中...");

    // 载入密文
    let ct_bytes = std::fs::read("ciphertext.bin")?;
    let ciphertext = Ciphertext::<Bn254>::deserialize_compressed(&ct_bytes[..])?;

    // 建立包含完整见证的电路
    let decrypt_circuit = SimpleCircuit {
        pub_input: Some(42),
        priv_input: Some(123), // 必须满足：42 + 123 = 165
    };

    // 解封装
    let recovered_key = decap::<Bn254, _>(decrypt_circuit, &ciphertext)?;

    println!("恢复的密钥：{:?}", recovered_key.as_bytes());

    // 验证密钥是否相符
    assert_eq!(key.as_bytes(), recovered_key.as_bytes());
    println!("✅ 密钥相符！");

    Ok(())
}
```

## 与其他工具整合

### 与 zkenc-cli 整合

zkenc-cli 内部使用 zkenc-core：

```rust
use zkenc_core::{encap, decap};
use zkenc_cli::circuit::CircomCircuit;

// 载入 R1CS 电路
let circuit = CircomCircuit::from_r1cs("circuit.r1cs")?;

// 封装
let (ciphertext, key) = encap::<Bn254, _, _>(circuit, &mut rng)?;
```

### 与自定义电路整合

```rust
// 您的自定义电路
impl ConstraintSynthesizer<Fr> for MyCircuit {
    fn generate_constraints(/* ... */) -> Result<(), SynthesisError> {
        // 您的约束
    }
}

// 与 zkenc-core 一起使用
let circuit = MyCircuit::new(/* 参数 */);
let (ct, key) = encap::<Bn254, _, _>(circuit, &mut rng)?;
```

## 效能提示

1. **电路最佳化**：最小化约束以加快操作速度
2. **批处理操作**：尽可能重复使用电路编译
3. **内存管理**：使用压缩序列化以减少大小
4. **平行处理**：encap/decap 可在多个消息之间平行化

## 曲线选择

目前，zkenc-core 默认使用 BN254 曲线：

```rust
use ark_bn254::Bn254;

let (ct, key) = encap::<Bn254, _, _>(circuit, &mut rng)?;
```

未来版本可能支持额外的曲线（BLS12-381 等）。

## 安全性考量

1. **RNG 安全性**：始终使用密码学安全的 RNG（`OsRng`）
2. **见证隐私**：永远不要暴露私密见证值
3. **电路正确性**：确保电路正确执行约束
4. **密钥使用**：使用适当的对称加密（AES-256-GCM）来使用密钥

## 下一步

- **[zkenc-cli API →](/docs/api/zkenc-cli)** - 命令行界面
- **[zkenc-js API →](/docs/api/zkenc-js)** - JavaScript 绑定
- **[快速入门 →](/docs/getting-started/zkenc-cli)** - 快速入门指南
