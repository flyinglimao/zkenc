---
sidebar_position: 1
---

# ガイド概要

zkenc ガイドへようこそ！これらのステップバイステップのチュートリアルは、Witness 暗号をプロジェクトに統合するのに役立ちます。

## 学べる内容

これらのガイドは、実際のアプリケーションで zkenc を使用する完全な実用例を提供します：

### 📦 Node.js 統合

Witness 暗号を使った完全な Node.js アプリケーションの構築方法を学びます。

- Circom 回路の読み込みとコンパイル
- ファイルの暗号化と復号
- 回路入力の適切な処理
- エラー処理とベストプラクティス

[Node.js ガイドを開始 →](/docs/guides/nodejs-integration)

### ⚛️ React 統合

Witness 暗号を使ったインタラクティブな React アプリケーションを構築します。

- Vite + React + TypeScript のセットアップ
- ブラウザでの回路ファイルの処理
- 暗号化/復号 UI の作成
- Web Workers によるパフォーマンス最適化

[React ガイドを開始 →](/docs/guides/react-integration)

### 🔄 クロスツールワークフロー

最大限の柔軟性を得るために zkenc-cli と zkenc-js を一緒に使用します。

- CLI で暗号化、JavaScript で復号
- 環境を超えて暗号文を共有
- ワークフローに合わせてツールの強みを組み合わせる
- バッチ処理と自動化

[クロスツールガイドを開始 →](/docs/guides/cross-tool-workflow)

## 前提条件

これらのガイドを開始する前に、以下が必要です：

1. **基本知識：**

   - JavaScript/TypeScript（JS ガイド用）
   - コマンドラインツール（CLI ガイド用）
   - Circom 回路（基本的な理解）

2. **必要なツールのインストール：**

   ```bash
   # Node.js (18+)
   node --version

   # Circom
   circom --version

   # zkenc-cli（クロスツールガイド用）
   zkenc --help
   ```

3. **回路の準備：**
   - `.circom` ソースファイル
   - またはプリコンパイルされた `.r1cs` と `.wasm` ファイル

## ガイドの構成

各ガイドは以下の構成に従います：

1. **セットアップ** - プロジェクトの初期化と依存関係
2. **回路の準備** - 回路のコンパイルと読み込み
3. **実装** - ステップバイステップのコード例
4. **テスト** - すべてが正常に動作することを確認
5. **最適化** - パフォーマンスの改善
6. **デプロイ** - 本番環境での考慮事項

## サンプル回路

ガイドではこれらのサンプル回路を使用します：

### シンプルなサンプル回路

学習用の基本回路：

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

### 数独回路

プレイグラウンドで使用される実用的な例：

```circom
pragma circom 2.0.0;

template Sudoku() {
    signal input puzzle[81];      // 公開：パズル
    signal input solution[81];    // 秘密：解答

    // 解答が有効であることを検証
    // ... 制約 ...
}

component main = Sudoku();
```

## 一般的なパターン

### 暗号化パターン

```typescript
// 1. 回路ファイルを読み込む
const circuitFiles = {
  r1csBuffer: await loadFile('circuit.r1cs'),
  wasmBuffer: await loadFile('circuit.wasm'),
};

// 2. 公開入力を準備
const publicInputs = { puzzle: [...] };

// 3. 暗号化
const { ciphertext } = await zkenc.encrypt(
  circuitFiles,
  publicInputs,
  message
);
```

### 復号パターン

```typescript
// 1. 暗号文を読み込む
const ciphertext = await loadFile('encrypted.bin');

// 2. 完全な入力を準備（公開 + 秘密）
const fullInputs = {
  puzzle: [...],
  solution: [...],
};

// 3. 復号
const decrypted = await zkenc.decrypt(
  circuitFiles,
  ciphertext,
  fullInputs
);
```

## ヘルプを得る

困ったときは：

1. **API リファレンスを確認：**

   - [zkenc-js API](/docs/api/zkenc-js)
   - [zkenc-cli API](/docs/api/zkenc-cli)
   - [zkenc-core API](/docs/api/zkenc-core)

2. **プレイグラウンドを試す：**

   - [インタラクティブな数独の例](/playground)

3. **サンプルコードを確認：**

   - 各ガイドには完全で実行可能な例が含まれています

4. **Issue を開く：**
   - [GitHub Issues](https://github.com/flyinglimao/zkenc/issues)

## ガイドを選択

<div className="guides-grid">

### Node.js 開発者向け

以下を構築している場合に最適：

- CLI ツール
- バックエンドサービス
- ファイル暗号化ツール
- バッチプロセッサ

[Node.js 統合 →](/docs/guides/nodejs-integration)

### React 開発者向け

以下を構築している場合に最適：

- Web アプリケーション
- インタラクティブな UI
- ブラウザベースのツール
- プログレッシブ Web アプリ

[React 統合 →](/docs/guides/react-integration)

### 自動化向け

以下の場合に最適：

- 複数のツールを使用
- ファイルのバッチ処理
- パイプラインの構築
- クロスプラットフォームワークフロー

[クロスツールワークフロー →](/docs/guides/cross-tool-workflow)

</div>

## 次のステップ

始める準備はできましたか？上記のガイドを選択するか：

- **初めての方？** [zkenc-js 入門](/docs/getting-started/zkenc-js)から始める
- **実験したい？** [プレイグラウンド](/playground)を試す
- **API の詳細が必要？** [API リファレンス](/docs/api/zkenc-js)を確認

Happy coding! 🚀
