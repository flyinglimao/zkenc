---
sidebar_position: 1
---

# zkenc-core APIリファレンス

zkenc-coreは、ウィットネス暗号化のための暗号プリミティブを提供するRustライブラリです。zkenc-cliとzkenc-jsの両方がこのコアライブラリの上に構築されている基盤層として機能します。

## 概要

zkenc-coreは、楕円曲線暗号(BN254曲線)を使用してR1CS回路のウィットネス暗号化を実装します。2つのコア関数を提供します:

- **`encap`**: ウィットネス暗号化されたキーを生成
- **`decap`**: 有効なウィットネスを使用してキーを回復

## インストール

`Cargo.toml`に追加:

```toml
[dependencies]
zkenc-core = { path = "../zkenc-core" }
ark-bn254 = "0.4"
ark-std = "0.4"
ark-serialize = "0.4"
```

## コア型

### `Ciphertext<E: Pairing>`

ウィットネス暗号化暗号文を表します。

```rust
pub struct Ciphertext<E: Pairing> {
    // 内部フィールドはプライベート
}
```

**プロパティ:**

- arkworksシリアライゼーションを使用してシリアライズ可能
- サイズ: BN254曲線で約1576バイト
- ネットワーク経由で送信またはディスクに保存可能

### `Key`

対称暗号化キーを表します。

```rust
pub struct Key {
    // 内部: 32バイトキー
}
```

**メソッド:**

- `as_bytes() -> &[u8; 32]` - AES暗号化用のキーをバイトとして取得

## コア関数

### `encap`

回路からウィットネス暗号化された暗号文とキーを生成します。

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

**型パラメータ:**

- `E` - 楕円曲線ペアリング(通常は`Bn254`)
- `C` - `ConstraintSynthesizer`を実装する回路
- `R` - 乱数生成器

**パラメータ:**

- `circuit` - 公開入力が設定された回路インスタンス
- `rng` - 暗号学的に安全な乱数生成器

**戻り値:**

- `Ok((Ciphertext, Key))` - 暗号文と生成されたキー
- `Err(EncapError)` - 回路合成が失敗した場合

**例:**

```rust
use zkenc_core::{encap, Ciphertext, Key};
use ark_bn254::Bn254;
use ark_std::rand::rngs::OsRng;

// 公開入力を持つ回路を作成
let mut circuit = MyCircuit::new();
circuit.set_public_input(42);

// 暗号文とキーを生成
let mut rng = OsRng;
let (ciphertext, key) = encap::<Bn254, _, _>(circuit, &mut rng)?;

// 保存/送信のためにシリアライズ
let mut ct_bytes = Vec::new();
ciphertext.serialize_compressed(&mut ct_bytes)?;
```

**パフォーマンス:**

- 時間計算量: O(n \* log n) ここでn = 制約の数
- メモリ: O(n) 制約システム用
- 一般的な時間: 回路サイズに応じて50-500ms

### `decap`

有効なウィットネスを使用して暗号化キーを回復します。

```rust
pub fn decap<E, C>(
    circuit: C,
    ciphertext: &Ciphertext<E>,
) -> Result<Key, DecapError>
where
    E: Pairing,
    C: ConstraintSynthesizer<E::ScalarField>,
```

**型パラメータ:**

- `E` - 楕円曲線ペアリング(通常は`Bn254`)
- `C` - `ConstraintSynthesizer`を実装する回路

**パラメータ:**

- `circuit` - 完全なウィットネスが設定された回路インスタンス
- `ciphertext` - `encap`からの暗号文

**戻り値:**

- `Ok(Key)` - 回復された暗号化キー
- `Err(DecapError)` - ウィットネスが無効または制約を満たさない場合

**例:**

```rust
use zkenc_core::{decap, Ciphertext, Key};
use ark_bn254::Bn254;
use ark_serialize::CanonicalDeserialize;

// 暗号文をロード
let ct_bytes = std::fs::read("ciphertext.bin")?;
let ciphertext = Ciphertext::<Bn254>::deserialize_compressed(&ct_bytes[..])?;

// 完全なウィットネスを持つ回路を作成
let mut circuit = MyCircuit::new();
circuit.set_public_input(42);
circuit.set_private_input(123); // ウィットネス

// キーを回復
let key = decap::<Bn254, _>(circuit, &ciphertext)?;

// 復号化にキーを使用
let key_bytes = key.as_bytes();
```

**パフォーマンス:**

- 時間計算量: O(n) ここでn = 制約の数
- メモリ: O(n) 制約システム用
- 一般的な時間: 回路サイズに応じて100-1000ms

## エラー型

### `EncapError`

カプセル化中に発生する可能性のあるエラー。

```rust
pub enum EncapError {
    /// 回路合成が失敗
    SynthesisError(SynthesisError),
    /// 乱数生成が失敗
    RngError,
}
```

### `DecapError`

デカプセル化中に発生する可能性のあるエラー。

```rust
pub enum DecapError {
    /// 回路合成が失敗
    SynthesisError(SynthesisError),
    /// ウィットネスが制約を満たさない
    InvalidWitness,
    /// ペアリングチェックが失敗
    PairingCheckFailed,
}
```

## 回路インターフェース

zkenc-coreを使用するには、回路が`ConstraintSynthesizer`を実装する必要があります:

```rust
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};

pub struct MyCircuit<F: Field> {
    pub public_input: Option<F>,
    pub private_input: Option<F>,
}

impl<F: Field> ConstraintSynthesizer<F> for MyCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        // 公開入力を割り当て
        let pub_var = cs.new_input_variable(|| {
            self.public_input.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // 秘密入力(ウィットネス)を割り当て
        let priv_var = cs.new_witness_variable(|| {
            self.private_input.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // 制約を追加
        // ... あなたの回路ロジック

        Ok(())
    }
}
```

## 完全な例

暗号化と復号化の完全な例:

```rust
use zkenc_core::{encap, decap};
use ark_bn254::Bn254;
use ark_ff::Field;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_r1cs_std::prelude::*;
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};
use ark_std::rand::rngs::OsRng;

// 回路を定義
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

        // 公開入力
        let pub_var = cs.new_input_variable(|| {
            self.pub_input.map(Fr::from).ok_or(SynthesisError::AssignmentMissing)
        })?;

        // 秘密ウィットネス
        let priv_var = cs.new_witness_variable(|| {
            self.priv_input.map(Fr::from).ok_or(SynthesisError::AssignmentMissing)
        })?;

        // 制約: pub + priv = 定数
        cs.enforce_constraint(
            lc!() + pub_var + priv_var,
            lc!() + Variable::One,
            lc!() + (165u64, Variable::One), // pub + priv = 165
        )?;

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 暗号化
    println!("暗号化中...");

    // 公開入力のみで回路を作成
    let encrypt_circuit = SimpleCircuit {
        pub_input: Some(42),
        priv_input: None, // 暗号化には不要
    };

    // カプセル化
    let mut rng = OsRng;
    let (ciphertext, key) = encap::<Bn254, _, _>(encrypt_circuit, &mut rng)?;

    println!("生成されたキー: {:?}", key.as_bytes());

    // 暗号文をシリアライズ
    let mut ct_bytes = Vec::new();
    ciphertext.serialize_compressed(&mut ct_bytes)?;
    std::fs::write("ciphertext.bin", &ct_bytes)?;

    // AES暗号化にキーを使用
    let message = b"Secret message";
    // ... key.as_bytes()でメッセージを暗号化 ...

    // 復号化
    println!("\n復号化中...");

    // 暗号文をロード
    let ct_bytes = std::fs::read("ciphertext.bin")?;
    let ciphertext = Ciphertext::<Bn254>::deserialize_compressed(&ct_bytes[..])?;

    // 完全なウィットネスで回路を作成
    let decrypt_circuit = SimpleCircuit {
        pub_input: Some(42),
        priv_input: Some(123), // 満たす必要: 42 + 123 = 165
    };

    // デカプセル化
    let recovered_key = decap::<Bn254, _>(decrypt_circuit, &ciphertext)?;

    println!("回復されたキー: {:?}", recovered_key.as_bytes());

    // キーが一致することを確認
    assert_eq!(key.as_bytes(), recovered_key.as_bytes());
    println!("✅ キーが一致しました!");

    Ok(())
}
```

## 他のツールとの統合

### zkenc-cliとの統合

zkenc-cliは内部的にzkenc-coreを使用します:

```rust
use zkenc_core::{encap, decap};
use zkenc_cli::circuit::CircomCircuit;

// R1CS回路をロード
let circuit = CircomCircuit::from_r1cs("circuit.r1cs")?;

// Encap
let (ciphertext, key) = encap::<Bn254, _, _>(circuit, &mut rng)?;
```

### カスタム回路との統合

```rust
// あなたのカスタム回路
impl ConstraintSynthesizer<Fr> for MyCircuit {
    fn generate_constraints(/* ... */) -> Result<(), SynthesisError> {
        // あなたの制約
    }
}

// zkenc-coreで使用
let circuit = MyCircuit::new(/* パラメータ */);
let (ct, key) = encap::<Bn254, _, _>(circuit, &mut rng)?;
```

## パフォーマンスのヒント

1. **回路の最適化**: より高速な操作のために制約を最小化
2. **バッチ操作**: 可能な場合は回路コンパイルを再利用
3. **メモリ管理**: サイズを削減するために圧縮シリアライゼーションを使用
4. **並列処理**: encap/decapは複数のメッセージにわたって並列化可能

## 曲線の選択

現在、zkenc-coreはデフォルトでBN254曲線を使用します:

```rust
use ark_bn254::Bn254;

let (ct, key) = encap::<Bn254, _, _>(circuit, &mut rng)?;
```

将来のバージョンでは追加の曲線(BLS12-381など)をサポートする可能性があります。

## セキュリティの考慮事項

1. **RNGセキュリティ**: 常に暗号学的に安全なRNG(`OsRng`)を使用
2. **ウィットネスのプライバシー**: 秘密ウィットネス値を公開しない
3. **回路の正確性**: 回路が適切に制約を強制することを確認
4. **キーの使用**: 適切な対称暗号化(AES-256-GCM)でキーを使用

## 次のステップ

- **[zkenc-cli API →](/docs/api/zkenc-cli)** - コマンドラインインターフェース
- **[zkenc-js API →](/docs/api/zkenc-js)** - JavaScriptバインディング
- **[入門 →](/docs/getting-started/zkenc-cli)** - クイックスタートガイド

