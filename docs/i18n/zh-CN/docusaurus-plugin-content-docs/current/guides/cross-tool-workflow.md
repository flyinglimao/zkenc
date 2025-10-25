---
sidebar_position: 4
---

# 跨工具工作流程指南

学习如何结合使用 zkenc-cli 和 zkenc-js，以获得最大的灵活性和功能。

## 为什么要同时使用两种工具？

结合 zkenc-cli 和 zkenc-js 可实现强大的工作流程：

- **服务器加密，浏览器解密**
- **CLI 用于批处理，JS 用于 UI**
- **不同环境，相同密文**
- **发挥每个工具的优势**

## 兼容性

zkenc-cli 和 zkenc-js **完全兼容**，使用相同的组合密文格式：

✅ 使用 CLI 加密的文件可以用 JS 解密
✅ 使用 JS 加密的文件可以用 CLI 解密
✅ 相同的电路文件可用于两种工具
✅ 两种工具使用相同的输入格式
✅ 不需要文件格式转换

**两种工具都使用相同的组合格式：**

```
[1 字节旗标][4 字节见证 CT 长度][见证密文]
[4 字节公开输入长度（如果旗标=1）][公开输入 JSON（如果旗标=1）]
[加密消息]
```

## 工作流程 1：CLI 加密 → JS 解密

**使用情境：**在服务器上加密敏感文件，在 Web 应用程序中解密。

### 步骤 1：准备电路 (CLI)

```bash
# 编译电路
circom circuit.circom --r1cs --wasm -o build

# 你需要：
# - build/circuit.r1cs（CLI 和 JS 都需要）
# - build/circuit_js/circuit.wasm（CLI 和 JS 都需要）
```

### 步骤 2：建立公开输入 (CLI)

建立 `public_inputs.json`：

```json
{
  "publicValue": "42"
}
```

### 步骤 3：使用 CLI 加密

```bash
# 一步完成加密（推荐）
zkenc encrypt \
  --circuit build/circuit.r1cs \
  --input public_inputs.json \
  --message secret.txt \
  --output encrypted.bin
```

输出的 `encrypted.bin` 是组合密文，包含：

- 见证加密密文
- 公开输入（默认嵌入）
- AES 加密消息

**文件大小：**

- `encrypted.bin`（组合）≈ 见证 CT（1576 字节）+ 公开输入 + 消息 + 额外开销

### 步骤 4：使用 JS 解密

```typescript
import { zkenc } from "zkenc-js";
import fs from "fs/promises";

// 载入组合密文
const ciphertext = await fs.readFile("encrypted.bin");

// 载入电路文件
const circuitFiles = {
  r1csBuffer: await fs.readFile("build/circuit.r1cs"),
  wasmBuffer: await fs.readFile("build/circuit_js/circuit.wasm"),
};

// 准备完整输入（公开 + 私密）
const fullInputs = {
  publicValue: 42,
  privateValue: 123, // 见证
};

// 一步完成解密
const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, fullInputs);

console.log(new TextDecoder().decode(decrypted));
// 输出：（secret.txt 的内容）
```

**就这样！**不需要文件转换。

## 工作流程 2：JS 加密 → CLI 解密

**使用情境：**在浏览器中加密，在服务器上处理/解密。

### 步骤 1：使用 JS 加密

```typescript
import { zkenc } from "zkenc-js";

const circuitFiles = {
  r1csBuffer: await fetch("/circuits/circuit.r1cs")
    .then((r) => r.arrayBuffer())
    .then((b) => new Uint8Array(b)),
  wasmBuffer: await fetch("/circuits/circuit.wasm")
    .then((r) => r.arrayBuffer())
    .then((b) => new Uint8Array(b)),
};

const publicInputs = { publicValue: "42" };
const message = new TextEncoder().encode("来自浏览器的秘密");

// 一步完成加密
const { ciphertext } = await zkenc.encrypt(circuitFiles, publicInputs, message);

// 下载密文
const blob = new Blob([ciphertext]);
const url = URL.createObjectURL(blob);
const a = document.createElement("a");
a.href = url;
a.download = "encrypted.bin";
a.click();
```

`ciphertext` 已经是 CLI 可以直接读取的组合格式。

### 步骤 2：生成见证 (CLI)

建立完整输入 `full_inputs.json`：

```json
{
  "publicValue": "42",
  "privateValue": "123"
}
```

使用 snarkjs 生成见证：

```bash
snarkjs wtns calculate \
  build/circuit_js/circuit.wasm \
  full_inputs.json \
  witness.wtns
```

### 步骤 3：使用 CLI 解密

```bash
# 一步完成解密
zkenc decrypt \
  --circuit build/circuit.r1cs \
  --witness witness.wtns \
  --ciphertext encrypted.bin \
  --output decrypted.txt

cat decrypted.txt
# 输出：来自浏览器的秘密
```

**就这样！**CLI 可以直接读取 JS 加密的文件。

## 工作流程 3：混合处理

**使用情境：**使用 CLI 进行批处理操作，使用 JS 进行交互式 UI。

### 范例：照片加密服务

**服务器 (CLI)：**

```bash
#!/bin/bash
# encrypt-photos.sh

for photo in uploads/*.jpg; do
  echo "正在处理 $photo..."

  # 生成唯一的公开输入
  PUBLIC_VALUE=$(date +%s)
  echo "{\"timestamp\": \"$PUBLIC_VALUE\"}" > inputs.json

  # 一步完成加密
  zkenc encrypt \
    --circuit circuit.r1cs \
    --input inputs.json \
    --message "$photo" \
    --output "${photo}.enc"

  # 存储元数据
  echo "$photo,$PUBLIC_VALUE" >> metadata.csv

  rm inputs.json
done
```

**客户端 (JS)：**

```typescript
// 解密选定的照片
async function decryptPhoto(photoId: string, privateValue: number) {
  // 获取加密照片（组合格式）
  const response = await fetch(`/api/photos/${photoId}/encrypted`);
  const ciphertext = new Uint8Array(await response.arrayBuffer());

  // 从元数据获取公开值
  const metadata = await fetch(`/api/photos/${photoId}/metadata`).then((r) =>
    r.json()
  );

  // 一步完成解密
  const fullInputs = {
    timestamp: metadata.timestamp,
    userSecret: privateValue,
  };

  const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, fullInputs);

  // 显示照片
  const blob = new Blob([decrypted], { type: "image/jpeg" });
  const url = URL.createObjectURL(blob);
  imageElement.src = url;
}
```

**注意：**如果嵌入了公开输入，可以使用 `getPublicInput()` 从密文中提取：

```typescript
import { getPublicInput } from "zkenc-js";

// 提取嵌入的公开输入
const publicInputs = getPublicInput(ciphertext);
console.log(publicInputs.timestamp); // 不需要获取元数据！
```

## 工作流程 4：多平台分发

**使用情境：**加密一次，在任何平台上解密。

### 设置

```bash
# 编译电路
circom puzzle.circom --r1cs --wasm -o dist

# 建立分发包
mkdir -p package/circuits
cp dist/puzzle.r1cs package/circuits/
cp dist/puzzle_js/puzzle.wasm package/circuits/
cp README.md package/
```

### 加密一次

```bash
# 建立谜题
cat > puzzle.json <<EOF
{
  "puzzle": ["5", "3", "0", "0", "7", "0", "0", "0", "0"]
}
EOF

# 加密消息（建立组合格式）
zkenc encrypt \
  --circuit package/circuits/puzzle.r1cs \
  --input puzzle.json \
  --message treasure.txt \
  --output package/treasure.enc
```

### 分发

```
package/
├── circuits/
│   ├── puzzle.r1cs     # 电路文件
│   └── puzzle.wasm      # 见证生成器
├── treasure.enc         # 组合密文（两种工具都能用！）
└── README.md            # 使用说明
```

### 使用者可以用任一工具解密

**CLI 使用者：**

```bash
# 生成解答见证
cat > solution.json <<EOF
{
  "puzzle": ["5", "3", "0", ...],
  "solution": ["5", "3", "4", "6", "7", "8", "9", "1", "2", ...]
}
EOF

snarkjs wtns calculate puzzle.wasm solution.json solution.wtns

# 直接解密
zkenc decrypt \
  --circuit puzzle.r1cs \
  --witness solution.wtns \
  --ciphertext treasure.enc \
  --output treasure.txt
```

**JS 使用者：**

```typescript
// 载入相同的加密文件
const ciphertext = await fetch('treasure.enc')
  .then(r => r.arrayBuffer())
  .then(b => new Uint8Array(b));

const solution = {
  puzzle: ["5", "3", "0", ...],
  solution: ["5", "3", "4", "6", "7", "8", "9", "1", "2", ...],
};

// 直接解密
const treasure = await zkenc.decrypt(circuitFiles, ciphertext, solution);
```

**不需要转换！**两种工具读取相同的文件格式。
## 进阶：使用低阶 API

对于进阶使用情境，你仍可以单独使用低阶 `encap`/`decap` 命令：

### CLI 低阶命令

```bash
# 步骤 1：生成见证密文和密钥
zkenc encap \
  --circuit circuit.r1cs \
  --input public.json \
  --ciphertext witness.ct \
  --key key.bin

# 步骤 2：使用任何 AES 工具或自定义实现加密
# （key.bin 是适用于 AES-256 的 32 字节密钥）

# 步骤 3：解密 - 恢复密钥
zkenc decap \
  --circuit circuit.r1cs \
  --witness solution.wtns \
  --ciphertext witness.ct \
  --key recovered_key.bin

# 步骤 4：使用步骤 2 中使用的相同方法解密
```

### 何时使用低阶 API

- 自定义加密方案
- 与现有加密管线整合
- 教学目的
- 单独调试加密/解密

**注意：**对于大多数使用情境，推荐使用高阶 `encrypt`/`decrypt` 命令。

## 最佳实践

1. **使用高阶 API**：使用 `encrypt`/`decrypt` 命令以确保简单性和兼容性
2. **保持电路文件一致**：跨工具使用相同的编译电路文件
3. **记录公开输入**：清楚记录哪些输入是公开的，哪些是私密的
4. **嵌入公开输入**：使用默认行为（嵌入）以获得自包含的密文
5. **版本控制你的电路**：追踪电路版本以确保兼容性
6. **双向测试**：始终测试 CLI→JS 和 JS→CLI 工作流程

## 疑难排解

**解密时出现「Invalid ciphertext」：**

- 确保文件是有效的 zkenc 密文（由 `encrypt` 命令建立）
- 验证文件在传输过程中没有损坏
- 检查你使用的是正确的电路文件

**「Witness doesn't satisfy constraints」：**

- 验证加密和解密之间的公开输入是否匹配
- 检查私密输入是否满足电路约束
- 确保使用相同的电路版本
- 使用 `snarkjs wtns check` 验证见证

**文件格式问题：**

- 文件已经兼容 - 不需要转换！
- 所有文件操作都使用二进制模式
- 避免使用可能损坏二进制文件的文本编辑器
- 如需检查文件，使用 `xxd` 或 `hexdump`

**公开输入不匹配：**

- CLI 和 JS 默认都会嵌入公开输入
- 在 JS 中使用 `getPublicInput()` 从密文提取
- CLI 在解密时会显示公开输入（如果已嵌入）

## 下一步

- **[Node.js 指南 →](/docs/guides/nodejs-integration)** - 建立 CLI 工具
- **[React 指南 →](/docs/guides/react-integration)** - 建立 Web UI
- **[API 参考 →](/docs/api/zkenc-js)** - 详细文件
- **[实验场 →](/playground)** - 在浏览器中试用
