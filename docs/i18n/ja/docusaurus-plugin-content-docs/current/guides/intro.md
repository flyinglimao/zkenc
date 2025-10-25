---
sidebar_position: 1
---

# ガイド概要

zkenc ガイドへようこそ!これらのステップバイステップのチュートリアルは、証拠暗号化をプロジェクトに統合する方法を説明します。

## 学習内容

これらのガイドは、実際のアプリケーションで zkenc を使用するための完全で実用的な例を提供します:

### 📦 Node.js 統合

証拠暗号化を使用した完全な Node.js アプリケーションの構築方法を学びます。

- Circom 回路のロードとコンパイル
- ファイルの暗号化と復号化
- 回路入力の適切な処理
- エラーハンドリングとベストプラクティス

[Node.js ガイドを始める →](/docs/guides/nodejs-integration)

### ⚛️ React 統合

証拠暗号化を使用したインタラクティブな React アプリケーションを構築します。

- Vite + React + TypeScript のセットアップ
- ブラウザでの回路ファイルの処理
- 暗号化/復号化 UI の作成
- Web Workers によるパフォーマンス最適化

[React ガイドを始める →](/docs/guides/react-integration)

### 🔄 クロスツールワークフロー

zkenc-cli と zkenc-js を組み合わせて、最大の柔軟性を実現します。

- CLI で暗号化、JavaScript で復号化
- 環境間での暗号文の共有
- ワークフローに合わせたツールの強みの組み合わせ
- バッチ処理と自動化

[クロスツールガイドを始める →](/docs/guides/cross-tool-workflow)

## 前提条件

これらのガイドを始める前に、以下を確認してください:

1. **基本的な知識:**

   - JavaScript/TypeScript(JS ガイド用)
   - コマンドラインツール(CLI ガイド用)
   - Circom 回路(基本的な理解)

2. **必要なツールのインストール:**

   ```bash
   # Node.js (18+)
   node --version

   # Circom
   circom --version

   # zkenc-cli (クロスツールガイド用)
   zkenc --help
   ```

3. **準備された回路:**
   - `.circom`ソースファイル
   - または事前コンパイルされた`.r1cs`と`.wasm`ファイル

## ガイドの構成

各ガイドは以下の構成に従います:

1. **セットアップ** - プロジェクトの初期化と依存関係
2. **回路の準備** - 回路のコンパイルとロード
3. **実装** - ステップバイステップのコード例
4. **テスト** - すべてが機能することを確認
5. **最適化** - パフォーマンスの改善
6. **デプロイ** - 本番環境での考慮事項

## サンプル回路

ガイドでは以下のサンプル回路を使用します:

### シンプルな例の回路

学習用の基本的な回路:

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

プレイグラウンドで使用される実用的な例:

```circom
pragma circom 2.0.0;

template Sudoku() {
    signal input puzzle[81];      // Public: パズル
    signal input solution[81];    // Private: 解答

    // 解答が有効であることを検証
    // ... 制約 ...
}

component main = Sudoku();
```

## 共通パターン

### 暗号化パターン

```typescript
// 1. 回路ファイルをロード
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

### 復号化パターン

```typescript
// 1. 暗号文をロード
const ciphertext = await loadFile('encrypted.bin');

// 2. 完全な入力を準備(公開 + 秘密)
const fullInputs = {
  puzzle: [...],
  solution: [...],
};

// 3. 復号化
const decrypted = await zkenc.decrypt(
  circuitFiles,
  ciphertext,
  fullInputs
);
```

## ヘルプの取得

問題が発生した場合:

1. **API リファレンスを確認:**

   - [zkenc-js API](/docs/api/zkenc-js)
   - [zkenc-cli API](/docs/api/zkenc-cli)
   - [zkenc-core API](/docs/api/zkenc-core)

2. **プレイグラウンドを試す:**

   - [インタラクティブな数独の例](/playground)

3. **サンプルコードを確認:**

   - 各ガイドには完全で実行可能な例が含まれています

4. **Issue を開く:**
   - [GitHub Issues](https://github.com/flyinglimao/zkenc/issues)

## ガイドを選択

<div className="guides-grid">

### Node.js 開発者向け

以下を構築する場合に最適:

- CLI ツール
- バックエンドサービス
- ファイル暗号化ツール
- バッチプロセッサ

[Node.js 統合 →](/docs/guides/nodejs-integration)

### React 開発者向け

以下を構築する場合に最適:

- Web アプリケーション
- インタラクティブな UI
- ブラウザベースのツール
- Progressive Web Apps

[React 統合 →](/docs/guides/react-integration)

### 自動化向け

以下の場合に最適:

- 複数のツールの使用
- ファイルのバッチ処理
- パイプラインの構築
- クロスプラットフォームワークフロー

[クロスツールワークフロー →](/docs/guides/cross-tool-workflow)

</div>

## 次のステップ

始める準備はできましたか?上記のガイドを選択するか、以下をご覧ください:

- **zkenc が初めての方は?** [zkenc-js 入門](/docs/getting-started/zkenc-js)から始めましょう
- **実験してみたい方は?** [プレイグラウンド](/playground)を試してください
- **API 詳細が必要な方は?** [API リファレンス](/docs/api/zkenc-js)を確認してください

ハッピーコーディング! 🚀
