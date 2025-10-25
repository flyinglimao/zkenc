---
sidebar_position: 2
---

# zkenc-cli 入门

zkenc-cli 是一个用于见证加密操作的命令行工具。它提供了一个简单的接口，可使用 Circom 电路加密和解密消息。

## 安装

### 从源代码安装

克隆仓库并从源代码构建：

```bash
git clone https://github.com/flyinglimao/zkenc.git
cd zkenc/packages/zkenc-cli
cargo install --path .
```

## 前置要求

使用 zkenc-cli 之前，您需要：

1. **已编译的 Circom 电路**，包含：

   - `.r1cs` 文件（电路约束）
   - `.wasm` 文件（见证生成器）

2. **输入文件**，格式为 JSON

## 快速开始

### 1. 创建简单电路

创建文件 `example.circom`：

```circom
pragma circom 2.0.0;

template Example() {
    signal input publicValue;
    signal input privateValue;
    signal output result;

    result <== publicValue + privateValue;
}

component main = Example();
```

### 2. 编译电路

```bash
circom example.circom --r1cs --wasm --output circuit_output
```

这将创建：

- `circuit_output/example.r1cs`
- `circuit_output/example_js/example.wasm`

### 3. 准备输入文件

创建 `public_inputs.json`（加密时已知）：

```json
{
  "publicValue": "42"
}
```

创建 `full_inputs.json`（解密时需要）：

```json
{
  "publicValue": "42",
  "privateValue": "123"
}
```

### 4. 加密您的秘密消息

使用 `encrypt` 一步完成见证加密：

```bash
echo "Hello, zkenc!" > message.txt
zkenc encrypt \
  --circuit circuit_output/example.r1cs \
  --input public_inputs.json \
  --message message.txt \
  --output encrypted.bin
```

此命令将：

- 从公开输入生成见证加密密钥（encap）
- 使用 AES-256-GCM 加密您的消息
- 将所有内容合并为单一密文文件
- 在密文中嵌入公开输入（默认）

输出：

```
🔐 Step 1: Running Encap...
📂 Loading R1CS circuit...
   - Constraints: 2
   - Public inputs: 1
   - Wires: 4

📋 Loading public inputs from JSON...
   - Parsed 1 field elements

   ✅ Witness ciphertext generated (123 bytes)

🔒 Step 2: Encrypting message...
   - Message size: 14 bytes
   ✅ Message encrypted (42 bytes)

📦 Step 3: Creating combined ciphertext...
   ✅ Combined ciphertext saved (218 bytes)

✨ Encryption complete! Public inputs are embedded in the ciphertext.
```

### 5. 生成见证文件

解密之前，接收者需要生成一个见证来证明他们拥有有效的解答：

```bash
snarkjs wtns calculate \
  circuit_output/example_js/example.wasm \
  full_inputs.json \
  witness.wtns
```

### 6. 解密消息

使用 `decrypt` 一步恢复并解密消息：

```bash
zkenc decrypt \
  --circuit circuit_output/example.r1cs \
  --witness witness.wtns \
  --ciphertext encrypted.bin \
  --output decrypted.txt
```

此命令将：

- 解析合并的密文
- 使用见证恢复密钥（decap）
- 使用 AES-256-GCM 解密消息

输出：

```
📦 Step 1: Parsing combined ciphertext...
   - Flag: 1
   - Witness ciphertext: 123 bytes
   - Public input: {"publicValue":"42"}
   - Encrypted message: 42 bytes

🔓 Step 2: Running Decap...
📂 Loading R1CS circuit...
   - Constraints: 2
   - Public inputs: 1

📋 Loading witness from snarkjs...
   - Witness elements: 4

   ✅ Key recovered from witness

🔓 Step 3: Decrypting message...
   ✅ Decrypted message saved (14 bytes)

✨ Decryption complete!
```

验证结果：

```bash
cat decrypted.txt
# 输出：Hello, zkenc!
```

## 命令参考

### `zkenc encap`

从电路和公开输入生成密文和加密密钥。

```bash
zkenc encap \
  --circuit <R1CS_FILE> \
  --input <JSON_FILE> \
  --ciphertext <OUTPUT_CT> \
  --key <OUTPUT_KEY>
```

**参数：**

- `--circuit <FILE>` - R1CS 电路文件的路径（来自 Circom 的 `.r1cs`）
- `--input <FILE>` - 包含公开输入的 JSON 文件路径
- `--ciphertext <FILE>` - 密文的输出路径
- `--key <FILE>` - 加密密钥的输出路径

**示例：**

```bash
zkenc encap \
  --circuit sudoku.r1cs \
  --input puzzle.json \
  --ciphertext ciphertext.bin \
  --key key.bin
```

---

### `zkenc decap`

使用有效的见证和密文恢复加密密钥。

```bash
zkenc decap \
  --circuit <R1CS_FILE> \
  --witness <WTNS_FILE> \
  --ciphertext <CT_FILE> \
  --key <OUTPUT_KEY>
```

**参数：**

- `--circuit <FILE>` - R1CS 电路文件的路径
- `--witness <FILE>` - 见证文件的路径（来自 snarkjs 的 `.wtns`）
- `--ciphertext <FILE>` - 密文文件的路径
- `--key <FILE>` - 恢复的密钥输出路径

**示例：**

```bash
zkenc decap \
  --circuit sudoku.r1cs \
  --witness solution.wtns \
  --ciphertext ciphertext.bin \
  --key recovered_key.bin
```

---

### `zkenc encrypt`

使用见证加密来加密消息（高级、一步操作）。

```bash
zkenc encrypt \
  --circuit <R1CS_FILE> \
  --input <JSON_FILE> \
  --message <MESSAGE_FILE> \
  --output <OUTPUT_FILE> \
  [--no-public-input]
```

**参数：**

- `--circuit <FILE>` - R1CS 电路文件的路径（来自 Circom 的 `.r1cs`）
- `--input <FILE>` - 包含公开输入的 JSON 文件路径
- `--message <FILE>` - 明文消息文件的路径
- `--output <FILE>` - 合并密文的输出路径
- `--no-public-input` - 不在密文中嵌入公开输入（可选）

**功能：**

此命令将 encap 和 AES 加密合并为单一步骤：

1. 从公开输入生成见证加密密钥
2. 使用 AES-256-GCM 加密消息
3. 创建合并密文，格式为：`[flag][witnessLen][witnessCT][publicLen][publicInput][encryptedMsg]`

**示例：**

```bash
zkenc encrypt \
  --circuit sudoku.r1cs \
  --input puzzle.json \
  --message secret.txt \
  --output encrypted.bin
```

**兼容性：** 输出与 zkenc-js 的 `decrypt()` 函数完全兼容。

---

### `zkenc decrypt`

使用见证解密来解密消息（高级、一步操作）。

```bash
zkenc decrypt \
  --circuit <R1CS_FILE> \
  --witness <WTNS_FILE> \
  --ciphertext <CT_FILE> \
  --output <OUTPUT_FILE>
```

**参数：**

- `--circuit <FILE>` - R1CS 电路文件的路径
- `--witness <FILE>` - 见证文件的路径（来自 snarkjs 的 `.wtns`）
- `--ciphertext <FILE>` - 合并密文文件的路径
- `--output <FILE>` - 解密消息的输出路径

**功能：**

此命令将 decap 和 AES 解密合并为单一步骤：

1. 解析合并的密文
2. 使用见证恢复密钥
3. 使用 AES-256-GCM 解密消息

**示例：**

```bash
zkenc decrypt \
  --circuit sudoku.r1cs \
  --witness solution.wtns \
  --ciphertext encrypted.bin \
  --output decrypted.txt
```

**兼容性：** 可以解密由 zkenc-js `encrypt()` 函数创建的文件。

---

### 低级命令

对于高级用例，您可以单独使用低级 encap/decap 命令：

#### `zkenc encap`

## 了解工作流程

zkenc-cli 提供两种级别的 API：

### 高级 API（推荐）

简单的两步骤流程：

1. **`encrypt`** - 在一个命令中合并 encap + AES 加密

   - 输入：电路、公开输入、消息
   - 输出：合并密文（与 zkenc-js 兼容）

2. **`decrypt`** - 在一个命令中合并 decap + AES 解密
   - 输入：电路、见证、合并密文
   - 输出：解密消息

**优点：**

- 更简单的工作流程（2 步骤相对于 4 步骤）
- 单一密文文件管理
- 与 zkenc-js 完全兼容
- 公开输入可嵌入密文中

### 低级 API（高级）

四步骤流程以进行精细控制：

1. **`encap`** - 从公开输入生成见证加密的密文和密钥
2. 单独加密消息（使用任何 AES 工具）
3. **`decap`** - 使用有效见证恢复密钥
4. 单独解密消息（使用任何 AES 工具）

**用例：**

- 自定义加密方案
- 跨多个消息重复使用密钥
- 与现有加密管道集成
- 教育目的以了解协议

**注意：** 对于大多数用例，建议使用高级 API，因为它确保兼容性并简化工作流程。

## 输入文件格式

### R1CS 电路文件（`.r1cs`）

由 Circom 编译器生成：

```bash
circom circuit.circom --r1cs --wasm --sym
```

### 见证文件（`.wtns`）

由 snarkjs 从您的完整输入生成：

```bash
# 从输入计算见证
snarkjs wtns calculate circuit.wasm input.json witness.wtns

# 验证见证（可选）
snarkjs wtns check circuit.r1cs witness.wtns
```

### 输入 JSON 文件

JSON 对象，以信号名称作为键：

```json
{
  "publicValue": "42",
  "privateValue": "123",
  "arraySignal": ["1", "2", "3"]
}
```

**重要注意事项：**

- 所有值必须是字符串（即使是数字）
- 支持数组信号
- 信号名称必须与电路中定义的名称匹配
- 对于 `encrypt`，仅提供公开输入
- 对于 `decrypt`，提供从完整输入（公开 + 私有）生成的见证文件

## 合并密文格式

`encrypt` 命令创建具有以下结构的合并密文：

```
[1 字节标志]
[4 字节见证密文长度]
[见证密文]
[4 字节公开输入长度]  （如果标志 = 1）
[公开输入 JSON]        （如果标志 = 1）
[加密消息]
```

**标志字节：**

- `1` = 包含公开输入（默认）
- `0` = 不包含公开输入（使用 `--no-public-input`）

此格式与 zkenc-js 兼容，并允许：

- 自包含密文（包含所有必要数据）
- 跨工具兼容性
- 可选的公开输入嵌入

## 处理二进制文件

### 加密二进制文件

您可以使用高级 API 加密任何文件类型：

```bash
# 一步加密图片
zkenc encrypt \
  --circuit circuit.r1cs \
  --input public.json \
  --message photo.jpg \
  --output encrypted_photo.bin

# 拥有见证的人一步解密图片
zkenc decrypt \
  --circuit circuit.r1cs \
  --witness solution.wtns \
  --ciphertext encrypted_photo.bin \
  --output decrypted_photo.jpg
```

### 使用低级 API 处理二进制文件

对于高级用例：

```bash
# 步骤 1：从电路生成密钥
zkenc encap \
  --circuit circuit.r1cs \
  --input public.json \
  --ciphertext witness_ct.bin \
  --key key.bin

# 步骤 2：使用外部工具或自定义方法加密
# （key.bin 是适用于 AES-256 的 32 字节密钥）

# 步骤 3：接收者恢复密钥
zkenc decap \
  --circuit circuit.r1cs \
  --witness solution.wtns \
  --ciphertext witness_ct.bin \
  --key recovered_key.bin

# 步骤 4：使用步骤 2 中使用的相同方法解密
```

## 高级用法

### 不嵌入公开输入的加密

默认情况下，`encrypt` 会在密文中嵌入公开输入。要排除它们：

```bash
zkenc encrypt \
  --circuit circuit.r1cs \
  --input public.json \
  --message message.txt \
  --output encrypted.bin \
  --no-public-input
```

**何时使用 `--no-public-input`：**

- 公开输入非常大
- 您将单独分发公开输入
- 您想要更小的密文文件

**注意：** 接收者需要公开输入来验证见证。

### 批处理

为相同的电路和公开输入加密多个消息：

```bash
# 使用嵌入的公开输入加密多个文件
for file in documents/*.txt; do
  zkenc encrypt \
    --circuit circuit.r1cs \
    --input public.json \
    --message "$file" \
    --output "encrypted/$(basename $file).enc"
done
```

每个加密文件都是自包含的，可以独立解密。

### 跨工具兼容性

zkenc-cli 与 zkenc-js **完全兼容**！您可以使用一个工具加密，用另一个工具解密：

**CLI → JS：**

```bash
# 使用 CLI 加密
zkenc encrypt \
  --circuit circuit.r1cs \
  --input public.json \
  --message message.txt \
  --output encrypted.bin

# 在 Node.js 或浏览器中使用 zkenc-js 解密
# encrypted.bin 可以由 zkenc-js decrypt() 读取
```

**JS → CLI：**

```bash
# 使用 zkenc-js encrypt() 加密后...
# 使用 CLI 解密
zkenc decrypt \
  --circuit circuit.r1cs \
  --witness solution.wtns \
  --ciphertext encrypted.bin \
  --output decrypted.txt
```

两个工具使用相同的合并密文格式，确保无缝互操作性。

[了解更多关于跨工具工作流程 →](/docs/guides/cross-tool-workflow)

## 性能提示

1. **使用高级 API**：`encrypt`/`decrypt` 命令有效处理一切
2. **嵌入公开输入**：保持密文自包含（默认行为）
3. **预先编译电路**：编译电路一次，多次重复使用
4. **考虑电路大小**：较大的电路 = 较慢的 encap/decap 操作
5. **二进制格式**：所有文件使用高效的二进制序列化

## 常见模式

### 条件访问控制

```bash
# 只有解决谜题的用户才能解密
zkenc encrypt \
  --circuit puzzle.r1cs \
  --input question.json \
  --message "Secret answer: 42" \
  --output secret.bin
```

### 时间锁加密

```bash
# 需要计算工作来生成见证
zkenc encrypt \
  --circuit timelock.r1cs \
  --input params.json \
  --message future_message.txt \
  --output locked.bin
```

### 分发加密文件

```bash
# 使用嵌入的公开输入加密
zkenc encrypt \
  --circuit circuit.r1cs \
  --input public.json \
  --message secret.txt \
  --output package.bin

# 公开分享 package.bin
# 只有能够生成有效见证的人才能解密
```

## 下一步

- **[API 参考 →](/docs/api/zkenc-cli)** - 完整的 CLI 命令参考
- **[跨工具工作流程 →](/docs/guides/cross-tool-workflow)** - 将 CLI 与 zkenc-js 一起使用
- **[zkenc-js 入门 →](/docs/getting-started/zkenc-js)** - JavaScript 替代方案

## 故障排除

### "Circuit file not found"（找不到电路文件）

确保 R1CS 文件路径正确：

```bash
# 检查文件是否存在
ls -lh circuit.r1cs
```

### "Invalid inputs"（无效输入）

检查您的 JSON 文件：

- 是有效的 JSON 格式
- 包含所有必需的信号名称
- 对所有数字使用字符串值

```bash
# 验证 JSON
cat inputs.json | jq .
```

### "Invalid ciphertext: too short"（无效密文：太短）

这表示密文文件已损坏或不是有效的 zkenc 密文。确保：

- 文件由 zkenc-cli `encrypt` 或 zkenc-js `encrypt()` 创建
- 文件未被修改或截断
- 您使用的是正确的文件

### "Decap failed"（Decap 失败）

这通常表示：

- 见证不满足电路约束
- 见证文件已损坏
- 使用错误的电路文件
- 见证与用于加密的公开输入不匹配

首先验证您的见证：

```bash
snarkjs wtns check circuit.r1cs witness.wtns
```

### "Decryption failed" 或 "Message decryption failed"（解密失败或消息解密失败）

确保：

- 见证满足电路约束
- 密文文件未损坏
- 使用正确的电路文件
- 见证与加密时的公开输入匹配

## 支持

如有问题或疑问：

1. 查看 [API 参考](/docs/api/zkenc-cli)
2. 查看[示例工作流程](/docs/guides/cross-tool-workflow)
3. 在 [GitHub](https://github.com/flyinglimao/zkenc) 上开启问题
