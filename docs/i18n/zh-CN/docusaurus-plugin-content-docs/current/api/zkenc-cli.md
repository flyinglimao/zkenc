---
sidebar_position: 2
---

# zkenc-cli API 参考

zkenc-cli 的完整命令行参考，这是基于 Rust 的见证加密工具。

## 安装

### 从源码建构

```bash
git clone https://github.com/flyinglimao/zkenc.git
cd zkenc
cargo build --release --package zkenc-cli

# 二进制文件位于: target/release/zkenc
```

### 添加至 PATH

```bash
# Linux/macOS
export PATH=$PATH:$(pwd)/target/release

# 或是安装至系统
sudo cp target/release/zkenc /usr/local/bin/
```

## 命令概览

zkenc-cli 提供四个主要命令：

| 命令      | 用途             | 输入               | 输出        |
| --------- | ---------------- | ------------------ | ----------- |
| `encap`   | 使用电路生成密钥 | R1CS + 公开输入    | 密文 + 密钥 |
| `decap`   | 使用见证恢复密钥 | R1CS + 见证 + 密文 | 密钥        |
| `encrypt` | 使用密钥加密消息 | 密钥 + 消息        | 加密文件    |
| `decrypt` | 使用密钥解密消息 | 密钥 + 加密文件    | 解密文件    |

## 命令

### `zkenc encap`

从电路和公开输入生成见证加密的密钥和密文。

```bash
zkenc encap [OPTIONS]
```

**必要选项：**

- `-c, --circuit <FILE>` - R1CS 电路文件路径 (.r1cs)
- `-i, --input <FILE>` - 包含公开输入的 JSON 文件路径
- `--ciphertext <FILE>` - 密文的输出路径
- `-k, --key <FILE>` - 加密密钥的输出路径

**范例：**

```bash
zkenc encap \
  --circuit sudoku.r1cs \
  --input public_inputs.json \
  --ciphertext witness.ct \
  --key encryption.key
```

**输入 JSON 格式：**

```json
{
  "puzzle": [5, 3, 0, 0, 7, 0, 0, 0, 0],
  "difficulty": 1
}
```

**输出：**

- **密文文件**: 约 1576 字节（见证加密密文）
- **密钥文件**: 约 32 字节（AES-256 加密密钥）

**范例输出：**

```
📂 Loading R1CS circuit...
   - Constraints: 12847
   - Public inputs: 81
   - Wires: 13129

📋 Loading public inputs from JSON...
   - Parsed 81 field elements

🔐 Running Encap...

💾 Saving ciphertext...
   ✅ Ciphertext saved (1576 bytes)

🔑 Saving key...
   ✅ Key saved (32 bytes)
```

### `zkenc decap`

使用有效见证恢复加密密钥。

```bash
zkenc decap [OPTIONS]
```

**必要选项：**

- `-c, --circuit <FILE>` - R1CS 电路文件路径 (.r1cs)
- `-w, --witness <FILE>` - 见证文件路径（来自 snarkjs 的 .wtns）
- `--ciphertext <FILE>` - 密文文件路径（来自 encap）
- `-k, --key <FILE>` - 恢复密钥的输出路径

**生成见证：**

首先，使用 snarkjs 生成见证：

```bash
# 建立完整输入 JSON（公开 + 私密）
cat > full_inputs.json <<EOF
{
  "puzzle": [5, 3, 0, 0, 7, 0, 0, 0, 0],
  "solution": [5, 3, 4, 6, 7, 8, 9, 1, 2]
}
EOF

# 使用 snarkjs 生成见证
snarkjs wtns calculate circuit.wasm full_inputs.json witness.wtns
```

**范例：**

```bash
zkenc decap \
  --circuit sudoku.r1cs \
  --witness witness.wtns \
  --ciphertext witness.ct \
  --key recovered.key
```

**输出：**

```
📂 Loading R1CS circuit...
   - Constraints: 12847
   - Public inputs: 81
   - Wires: 13129

📂 Loading witness...
   - Wires: 13129

📂 Loading ciphertext...
   - Size: 1576 bytes

🔓 Running Decap...
   ✅ Key recovered successfully

💾 Saving recovered key...
   ✅ Key saved (32 bytes)
```

### `zkenc encrypt`

使用加密密钥加密消息。

```bash
zkenc encrypt [OPTIONS]
```

**必要选项：**

- `-k, --key <FILE>` - 加密密钥文件路径（来自 encap 或 decap）
- `-i, --input <FILE>` - 明文文件路径
- `-o, --output <FILE>` - 输出加密文件路径

**范例：**

```bash
# 加密文本文件
zkenc encrypt \
  --key encryption.key \
  --input message.txt \
  --output message.txt.enc

# 加密二进制文件
zkenc encrypt \
  --key encryption.key \
  --input document.pdf \
  --output document.pdf.enc
```

**输出：**

```
🔑 Loading key...
📄 Loading plaintext...
   - Plaintext size: 1234 bytes

🔒 Encrypting...
   ✅ Encrypted file saved (1266 bytes)
```

**注意：** 输出大小 = 输入大小 + 28 字节（GCM nonce + tag）

### `zkenc decrypt`

使用加密密钥解密消息。

```bash
zkenc decrypt [OPTIONS]
```

**必要选项：**

- `-k, --key <FILE>` - 加密密钥文件路径
- `-i, --input <FILE>` - 加密文件路径
- `-o, --output <FILE>` - 输出解密文件路径

**范例：**

```bash
zkenc decrypt \
  --key recovered.key \
  --input message.txt.enc \
  --output decrypted.txt
```

**输出：**

```
🔑 Loading key...
📦 Loading encrypted data...
   - Encrypted size: 1266 bytes

🔓 Decrypting...
   ✅ Decrypted file saved (1234 bytes)
```
## 完整工作流程

### 完整的加密/解密流程

```bash
# 1. 封装：使用电路生成密钥
zkenc encap \
  --circuit circuit.r1cs \
  --input public.json \
  --ciphertext witness.ct \
  --key encryption.key

# 2. 加密：使用密钥加密消息
zkenc encrypt \
  --key encryption.key \
  --input secret.txt \
  --output secret.txt.enc

# 3. 使用 snarkjs 生成见证
snarkjs wtns calculate circuit.wasm full_inputs.json witness.wtns

# 4. 解封装：使用见证恢复密钥
zkenc decap \
  --circuit circuit.r1cs \
  --witness witness.wtns \
  --ciphertext witness.ct \
  --key recovered.key

# 5. 解密：使用恢复的密钥解密消息
zkenc decrypt \
  --key recovered.key \
  --input secret.txt.enc \
  --output decrypted.txt
```

### 简化流程（单步）

为方便起见，您可以结合 encap + encrypt：

```bash
# 加密（在一个脚本中执行 encap + encrypt）
zkenc encap \
  --circuit circuit.r1cs \
  --input public.json \
  --ciphertext witness.ct \
  --key temp.key

zkenc encrypt \
  --key temp.key \
  --input message.txt \
  --output message.enc

# 分发: witness.ct + message.enc

# 解密（在一个脚本中执行 decap + decrypt）
snarkjs wtns calculate circuit.wasm full_inputs.json witness.wtns

zkenc decap \
  --circuit circuit.r1cs \
  --witness witness.wtns \
  --ciphertext witness.ct \
  --key temp.key

zkenc decrypt \
  --key temp.key \
  --input message.enc \
  --output decrypted.txt
```

## 文件格式

### 输入 JSON 格式

**公开输入（用于 encap）：**

```json
{
  "signalName1": 42,
  "signalName2": [1, 2, 3],
  "signalName3": "123"
}
```

**完整输入（用于见证生成）：**

```json
{
  "publicSignal": [5, 3, 0],
  "privateSignal": [5, 3, 4]
}
```

**规则：**

- 数字可以是整数或字符串
- 数组会自动展平
- 按排序顺序处理键
- 所有值必须是有效的字段元素

### 电路文件

**必要文件：**

- `.r1cs` - R1CS 电路文件（来自 circom 编译）
- `.wasm` - WASM 见证生成器（用于 snarkjs）

**编译电路：**

```bash
circom circuit.circom --r1cs --wasm --output build
# 生成: build/circuit.r1cs, build/circuit_js/circuit.wasm
```

### 见证文件

**格式：** `.wtns`（snarkjs 二进制格式）

**生成：**

```bash
snarkjs wtns calculate circuit.wasm inputs.json witness.wtns
```

### 输出文件

- **密文** (`.ct`): 约 1576 字节，见证加密密文
- **密钥** (`.key`): 约 32 字节，AES-256 加密密钥
- **加密文件** (`.enc`): 原始大小 + 28 字节，AES-256-GCM 密文

## 整合范例

### Bash 脚本

```bash
#!/bin/bash
set -e

CIRCUIT="sudoku.r1cs"
WASM="sudoku.wasm"
PUBLIC="public.json"
FULL="full_inputs.json"
MESSAGE="secret.txt"

echo "加密中..."
zkenc encap -c "$CIRCUIT" -i "$PUBLIC" --ciphertext ct.bin -k key.bin
zkenc encrypt -k key.bin -i "$MESSAGE" -o encrypted.bin

echo "解密中..."
snarkjs wtns calculate "$WASM" "$FULL" witness.wtns
zkenc decap -c "$CIRCUIT" -w witness.wtns --ciphertext ct.bin -k recovered.bin
zkenc decrypt -k recovered.bin -i encrypted.bin -o decrypted.txt

echo "验证..."
diff "$MESSAGE" decrypted.txt && echo "✅ 成功！"
```

### Make 整合

```makefile
.PHONY: encrypt decrypt clean

CIRCUIT := circuit.r1cs
WASM := circuit.wasm
PUBLIC := public.json
FULL := full.json

encrypt: message.txt
	zkenc encap -c $(CIRCUIT) -i $(PUBLIC) --ciphertext witness.ct -k encrypt.key
	zkenc encrypt -k encrypt.key -i message.txt -o message.enc
	@echo "已加密: witness.ct + message.enc"

decrypt: witness.ct message.enc
	snarkjs wtns calculate $(WASM) $(FULL) witness.wtns
	zkenc decap -c $(CIRCUIT) -w witness.wtns --ciphertext witness.ct -k decrypt.key
	zkenc decrypt -k decrypt.key -i message.enc -o decrypted.txt
	@echo "已解密: decrypted.txt"

clean:
	rm -f *.ct *.key *.enc *.wtns decrypted.txt
```

## 跨工具兼容性

zkenc-cli 与 zkenc-js 完全兼容。文件可以在它们之间共享。

### CLI 加密 → JS 解密

```bash
# CLI: 加密
zkenc encap -c circuit.r1cs -i public.json --ciphertext witness.ct -k key.bin
zkenc encrypt -k key.bin -i message.txt -o message.enc

# 为 zkenc-js 合并文件
cat <(head -c 4 <(printf '\x00\x00\x06(\n')) witness.ct message.enc > combined.bin
```

```javascript
// JS: 解密
import { zkenc } from "zkenc-js";
import fs from "fs/promises";

const ciphertext = await fs.readFile("combined.bin");
const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, fullInputs);
```

[深入了解跨工具工作流程 →](/docs/guides/cross-tool-workflow)

## 效能

### Encap 效能

| 电路大小   | 约束数量         | 时间       |
| ---------- | ---------------- | ---------- |
| 小型       | < 1,000          | < 100ms    |
| 中型       | 1,000 - 10,000   | 100ms - 1s |
| 大型       | 10,000 - 100,000 | 1s - 10s   |
| 超大型     | > 100,000        | > 10s      |

### Decap 效能

类似于 encap，加上见证计算开销（约 50-200ms）

### Encrypt/Decrypt 效能

非常快（< 10ms）- 仅 AES 操作，与电路大小无关
## 疑难排解

### "Failed to load R1CS circuit"

- 检查文件路径是否正确
- 确保文件是有效的 R1CS 格式（使用 circom 编译）
- 尝试重新编译电路

```bash
circom circuit.circom --r1cs --output build
```

### "Failed to parse input JSON"

- 验证 JSON 语法
- 确保所有值是数字或字符串
- 检查信号名称是否符合电路

```bash
# 验证 JSON
cat inputs.json | jq .
```

### "Decap failed"

- 见证不满足电路约束
- 错误的电路文件
- 密文已损坏

**调试：**

```bash
# 测试见证生成
snarkjs wtns calculate circuit.wasm inputs.json test.wtns

# 检查电路
snarkjs r1cs info circuit.r1cs
```

### "Encryption/Decryption failed"

- 错误的密钥文件
- 加密文件已损坏
- 文件格式不符

**验证密钥：**

```bash
# 密钥应该刚好是 32 字节
ls -l *.key
```

## 最佳实践

1. **保持电路文件安全**：加密和解密都需要 R1CS 文件
2. **分离公开/私密输入**：仅与加密者共享公开输入
3. **验证见证有效性**：解密前测试见证生成
4. **使用一致的文件命名**：遵循惯例（`.ct`、`.key`、`.enc`）
5. **临时备份密钥**：密钥仅在加密阶段需要

## 安全性考量

- **密钥管理**：密钥是临时的 - 应安全保管见证
- **电路完整性**：确保 R1CS 文件未被篡改
- **见证隐私**：永远不要共享见证文件 - 它们就像私钥
- **传输安全**：使用安全通道分发密文

## 下一步

- **[快速入门 →](/docs/getting-started/zkenc-cli)** - 快速入门指南
- **[跨工具工作流程 →](/docs/guides/cross-tool-workflow)** - 与 zkenc-js 一起使用
- **[zkenc-core API →](/docs/api/zkenc-core)** - Rust 库参考
